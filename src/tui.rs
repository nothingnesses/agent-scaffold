//! The interactive two-pane principle selector (TEA-lite).
//!
//! The Elm split is expressed as plain functions over one concrete `App` value:
//!
//! - `update(&mut App, Event) -> Step` is a pure-ish reducer (no IO), so the
//!   toggle, reorder, focus, and confirm logic is unit-tested without a
//!   terminal.
//! - `ui(&mut Frame, &mut App)` renders as a function of state; `App` does not
//!   implement `Widget`.
//! - `run` wires read -> update -> draw and is the only impure part.
//!
//! Events come through one `next_event()` seam (currently a blocking
//! `event::read()`). If a concurrent producer is ever needed, that seam becomes
//! a channel receiver without changing `update` or `ui`.

use {
	crate::pack::Principle,
	ratatui::{
		Frame,
		crossterm::event::{
			self,
			Event as CtEvent,
			KeyCode,
			KeyEvent,
			KeyEventKind,
			KeyModifiers,
		},
		layout::{
			Constraint,
			Layout,
			Rect,
		},
		style::{
			Color,
			Modifier,
			Style,
			Stylize,
		},
		text::{
			Line,
			Span,
			Text,
		},
		widgets::{
			Block,
			List,
			ListItem,
			ListState,
			Paragraph,
			Wrap,
		},
	},
	std::io,
};

/// Which pane holds the cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pane {
	Available,
	Included,
}

/// An input event the loop reacts to. `Resize` triggers a redraw; the loop
/// redraws every iteration, so it carries no payload. This enum is the seam a
/// future concurrent producer would extend.
enum Event {
	Key(KeyEvent),
	Resize,
}

/// What the reducer tells the loop to do next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
	/// Keep looping.
	Continue,
	/// Confirm the current selection and order.
	Confirm,
	/// Abort without writing anything.
	Abort,
}

/// The whole selector state.
///
/// `included` (ordered) and `available` are disjoint and together cover every
/// principle index exactly once. The only mutations are `toggle` (move one
/// index between the two) and `reorder` (swap adjacent within `included`), so
/// the invariant holds by construction.
struct App<'a> {
	principles: &'a [Principle],
	/// Included principle indices, in the user's chosen order.
	included: Vec<usize>,
	/// Available (not included) indices, seeded in `default_order`.
	available: Vec<usize>,
	focus: Pane,
	available_state: ListState,
	included_state: ListState,
}

impl<'a> App<'a> {
	fn new(
		principles: &'a [Principle],
		initial_included: &[usize],
	) -> Self {
		let included: Vec<usize> = initial_included.to_vec();
		let mut available: Vec<usize> =
			(0 .. principles.len()).filter(|i| !included.contains(i)).collect();
		available.sort_by_key(|&i| principles[i].default_order);

		let mut available_state = ListState::default();
		if !available.is_empty() {
			available_state.select(Some(0));
		}
		let mut included_state = ListState::default();
		if !included.is_empty() {
			included_state.select(Some(0));
		}

		Self {
			principles,
			included,
			available,
			focus: Pane::Available,
			available_state,
			included_state,
		}
	}

	fn focused_len(&self) -> usize {
		match self.focus {
			Pane::Available => self.available.len(),
			Pane::Included => self.included.len(),
		}
	}

	fn focused_state(&mut self) -> &mut ListState {
		match self.focus {
			Pane::Available => &mut self.available_state,
			Pane::Included => &mut self.included_state,
		}
	}

	/// Keep the focused pane's selection within bounds after a mutation.
	fn clamp_focused(&mut self) {
		let len = self.focused_len();
		let state = self.focused_state();
		match len {
			0 => state.select(None),
			_ => {
				let clamped = state.selected().unwrap_or(0).min(len - 1);
				state.select(Some(clamped));
			}
		}
	}

	fn cursor_down(&mut self) {
		let len = self.focused_len();
		if len == 0 {
			return;
		}
		let state = self.focused_state();
		// Wrap past the bottom back to the top.
		let next = match state.selected() {
			Some(i) if i + 1 < len => i + 1,
			_ => 0,
		};
		state.select(Some(next));
	}

	fn cursor_up(&mut self) {
		let len = self.focused_len();
		if len == 0 {
			return;
		}
		let state = self.focused_state();
		// Wrap past the top round to the bottom.
		let prev = match state.selected() {
			Some(i) if i > 0 => i - 1,
			_ => len - 1,
		};
		state.select(Some(prev));
	}

	fn switch_focus(&mut self) {
		self.focus = match self.focus {
			Pane::Available => Pane::Included,
			Pane::Included => Pane::Available,
		};
		let len = self.focused_len();
		let state = self.focused_state();
		match len {
			0 => state.select(None),
			_ if state.selected().is_none_or(|i| i >= len) => state.select(Some(0)),
			_ => {}
		}
	}

	/// Move the highlighted principle to the other pane. `insert` decides where
	/// it lands relative to that pane's cursor (before or after), and leaves the
	/// destination cursor on the moved item.
	fn toggle_with(
		&mut self,
		insert: fn(&mut Vec<usize>, &mut ListState, usize),
	) {
		let len = self.focused_len();
		let Some(pos) = self.focused_state().selected() else {
			return;
		};
		if pos >= len {
			return;
		}
		match self.focus {
			Pane::Available => {
				let idx = self.available.remove(pos);
				insert(&mut self.included, &mut self.included_state, idx);
			}
			Pane::Included => {
				let idx = self.included.remove(pos);
				insert(&mut self.available, &mut self.available_state, idx);
			}
		}
		self.clamp_focused();
		debug_assert_eq!(
			self.included.len() + self.available.len(),
			self.principles.len(),
			"included and available must partition the principles"
		);
	}

	/// Move the highlighted included principle one place towards the front.
	fn reorder_up(&mut self) {
		if self.focus != Pane::Included {
			return;
		}
		let Some(pos) = self.included_state.selected() else {
			return;
		};
		if pos == 0 || pos >= self.included.len() {
			return;
		}
		self.included.swap(pos, pos - 1);
		self.included_state.select(Some(pos - 1));
	}

	/// Move the highlighted included principle one place towards the back.
	fn reorder_down(&mut self) {
		if self.focus != Pane::Included {
			return;
		}
		let Some(pos) = self.included_state.selected() else {
			return;
		};
		if pos + 1 >= self.included.len() {
			return;
		}
		self.included.swap(pos, pos + 1);
		self.included_state.select(Some(pos + 1));
	}

	/// The principle currently under the focused cursor, if any.
	fn highlighted(&self) -> Option<&Principle> {
		let (indices, state) = match self.focus {
			Pane::Available => (&self.available, &self.available_state),
			Pane::Included => (&self.included, &self.included_state),
		};
		let pos = state.selected()?;
		indices.get(pos).map(|&idx| &self.principles[idx])
	}

	/// The detail-footer text for the highlighted principle.
	fn detail_text(&self) -> Text<'static> {
		let Some(p) = self.highlighted() else {
			return Text::from("No principle highlighted.");
		};
		let mut lines =
			vec![Line::from(Span::from(p.name.clone()).bold()), Line::from(p.summary.clone())];
		if !p.tags.is_empty() {
			lines.push(Line::from(format!("tags: {}", p.tags.join(", "))).dim());
		}
		Text::from(lines)
	}
}

/// Insert `idx` into `dest` just before its cursor and move the cursor onto the
/// inserted item. An empty destination takes the item at the front.
fn insert_before_cursor(
	dest: &mut Vec<usize>,
	state: &mut ListState,
	idx: usize,
) {
	let at = match state.selected() {
		Some(cursor) => cursor.min(dest.len()),
		None => dest.len(),
	};
	dest.insert(at, idx);
	state.select(Some(at));
}

/// Insert `idx` into `dest` just after its cursor and move the cursor onto the
/// inserted item. An empty destination takes the item at the front.
fn insert_after_cursor(
	dest: &mut Vec<usize>,
	state: &mut ListState,
	idx: usize,
) {
	let at = match state.selected() {
		Some(cursor) => (cursor + 1).min(dest.len()),
		None => dest.len(),
	};
	dest.insert(at, idx);
	state.select(Some(at));
}

/// The reducer: apply one event to the state and report what the loop should do.
fn update(
	app: &mut App,
	event: Event,
) -> Step {
	let key = match event {
		Event::Key(key) => key,
		Event::Resize => return Step::Continue,
	};

	match key.code {
		KeyCode::Char('q') | KeyCode::Esc => return Step::Abort,
		KeyCode::Enter => return Step::Confirm,
		KeyCode::Char('i') => app.toggle_with(insert_before_cursor),
		KeyCode::Char('a') => app.toggle_with(insert_after_cursor),
		KeyCode::Tab
		| KeyCode::BackTab
		| KeyCode::Left
		| KeyCode::Right
		| KeyCode::Char('h')
		| KeyCode::Char('l') => app.switch_focus(),
		KeyCode::Up if key.modifiers.contains(KeyModifiers::SHIFT) => app.reorder_up(),
		KeyCode::Down if key.modifiers.contains(KeyModifiers::SHIFT) => app.reorder_down(),
		KeyCode::Char('K') => app.reorder_up(),
		KeyCode::Char('J') => app.reorder_down(),
		KeyCode::Up | KeyCode::Char('k') => app.cursor_up(),
		KeyCode::Down | KeyCode::Char('j') => app.cursor_down(),
		_ => {}
	}
	Step::Continue
}

/// The fixed presentation of a pane (its varying-per-pane, non-state fields).
struct PaneSpec<'a> {
	title: &'a str,
	focused: bool,
	numbered: bool,
}

/// Render one pane's list into `area`.
fn render_pane(
	frame: &mut Frame,
	area: Rect,
	principles: &[Principle],
	indices: &[usize],
	state: &mut ListState,
	spec: PaneSpec,
) {
	let PaneSpec {
		title,
		focused,
		numbered,
	} = spec;
	let items: Vec<ListItem> = indices
		.iter()
		.enumerate()
		.map(|(n, &idx)| {
			let name = &principles[idx].name;
			let label = if numbered { format!("{}. {name}", n + 1) } else { name.clone() };
			ListItem::new(label)
		})
		.collect();

	let border = if focused {
		Style::default().fg(Color::Yellow)
	} else {
		Style::default().fg(Color::DarkGray)
	};
	let block =
		Block::bordered().title(format!("{title} ({})", indices.len())).border_style(border);
	let highlight =
		if focused { Style::default().add_modifier(Modifier::REVERSED) } else { Style::default() };
	let list = List::new(items)
		.block(block)
		.highlight_style(highlight)
		.highlight_symbol(if focused { "> " } else { "  " });
	frame.render_stateful_widget(list, area, state);
}

/// Render the whole selector.
fn ui(
	frame: &mut Frame,
	app: &mut App,
) {
	let detail = app.detail_text();

	let [content_area, detail_area, help_area] =
		Layout::vertical([Constraint::Fill(1), Constraint::Length(6), Constraint::Length(1)])
			.areas(frame.area());
	let [available_area, included_area] =
		Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
			.areas(content_area);

	render_pane(
		frame,
		available_area,
		app.principles,
		&app.available,
		&mut app.available_state,
		PaneSpec {
			title: "Available",
			focused: app.focus == Pane::Available,
			numbered: false,
		},
	);
	render_pane(
		frame,
		included_area,
		app.principles,
		&app.included,
		&mut app.included_state,
		PaneSpec {
			title: "Included",
			focused: app.focus == Pane::Included,
			numbered: true,
		},
	);

	frame.render_widget(
		Paragraph::new(detail)
			.wrap(Wrap {
				trim: true,
			})
			.block(Block::bordered().title("Details")),
		detail_area,
	);
	frame.render_widget(
		Paragraph::new(
			"i/a insert before/after  tab/h/l focus  j/k cursor  J/K reorder  enter confirm  q abort",
		)
		.centered()
		.dim(),
		help_area,
	);
}

/// Read the next event the loop cares about, ignoring the rest.
fn next_event() -> io::Result<Event> {
	loop {
		match event::read()? {
			CtEvent::Key(key) if key.kind == KeyEventKind::Press => return Ok(Event::Key(key)),
			CtEvent::Resize(..) => return Ok(Event::Resize),
			_ => {}
		}
	}
}

/// The read -> update -> draw loop. Returns the confirmed included order, or
/// `None` if the user aborted.
fn run(
	terminal: &mut ratatui::DefaultTerminal,
	principles: &[Principle],
	initial_included: &[usize],
) -> io::Result<Option<Vec<usize>>> {
	let mut app = App::new(principles, initial_included);
	loop {
		terminal.draw(|frame| ui(frame, &mut app))?;
		match update(&mut app, next_event()?) {
			Step::Continue => {}
			Step::Confirm => return Ok(Some(app.included.clone())),
			Step::Abort => return Ok(None),
		}
	}
}

/// Run the interactive selector. Returns the chosen principle indices (into
/// `principles`) in the confirmed order, or `None` if the user aborted.
///
/// `initial_included` seeds the included pane (in order); everything else starts
/// in the available pane ordered by `default_order`.
pub fn run_selection(
	principles: &[Principle],
	initial_included: &[usize],
) -> io::Result<Option<Vec<usize>>> {
	let mut terminal = ratatui::init();
	let outcome = run(&mut terminal, principles, initial_included);
	ratatui::restore();
	outcome
}

#[cfg(test)]
mod tests {
	use super::*;

	fn principle(
		id: &str,
		order: i64,
	) -> Principle {
		Principle {
			id: id.to_string(),
			name: id.to_string(),
			summary: String::new(),
			rationale: String::new(),
			tags: vec!["t".to_string()],
			default_selected: false,
			default_order: order,
			references: Vec::new(),
			related: Vec::new(),
		}
	}

	fn sample() -> Vec<Principle> {
		vec![principle("a", 10), principle("b", 20), principle("c", 30)]
	}

	fn key(code: KeyCode) -> Event {
		Event::Key(KeyEvent::new(code, KeyModifiers::NONE))
	}

	#[test]
	fn toggle_moves_between_panes_and_keeps_the_partition() {
		let principles = sample();
		let mut app = App::new(&principles, &[]);
		assert_eq!(app.available, vec![0, 1, 2]);
		assert!(app.included.is_empty());

		// Include the highlighted item ('a').
		assert_eq!(update(&mut app, key(KeyCode::Char('i'))), Step::Continue);
		assert_eq!(app.included, vec![0]);
		assert_eq!(app.available, vec![1, 2]);
		assert_eq!(app.included.len() + app.available.len(), principles.len());

		// Exclude it again from the Included pane; the partition still holds.
		update(&mut app, key(KeyCode::Tab));
		assert_eq!(app.focus, Pane::Included);
		update(&mut app, key(KeyCode::Char('i')));
		assert!(app.included.is_empty());
		assert_eq!(app.available.len(), 3);
	}

	#[test]
	fn i_inserts_before_the_destination_cursor() {
		let principles = sample();
		let mut app = App::new(&principles, &[1, 2]);
		assert_eq!(app.available, vec![0]);

		// The destination (Included) cursor sits on its second row; `i` moves
		// the item just before it, and the cursor follows to the moved item.
		app.included_state.select(Some(1));
		update(&mut app, key(KeyCode::Char('i')));
		assert_eq!(app.included, vec![1, 0, 2]);
		assert_eq!(app.included_state.selected(), Some(1));
	}

	#[test]
	fn a_inserts_after_the_destination_cursor() {
		let principles = sample();
		let mut app = App::new(&principles, &[1, 2]);
		assert_eq!(app.available, vec![0]);

		// With the destination cursor on its first row, `a` moves the item just
		// after it, and the cursor follows to the moved item.
		app.included_state.select(Some(0));
		update(&mut app, key(KeyCode::Char('a')));
		assert_eq!(app.included, vec![1, 0, 2]);
		assert_eq!(app.included_state.selected(), Some(1));
	}

	#[test]
	fn reorder_swaps_within_included_and_clamps_at_the_ends() {
		let principles = sample();
		let mut app = App::new(&principles, &[0, 1, 2]);
		update(&mut app, key(KeyCode::Tab));
		assert_eq!(app.focus, Pane::Included);
		assert_eq!(app.included_state.selected(), Some(0));

		// At the top, reorder-up is a no-op.
		update(&mut app, key(KeyCode::Char('K')));
		assert_eq!(app.included, vec![0, 1, 2]);

		// Move the top item down; the cursor follows it.
		update(&mut app, key(KeyCode::Char('J')));
		assert_eq!(app.included, vec![1, 0, 2]);
		assert_eq!(app.included_state.selected(), Some(1));
	}

	#[test]
	fn cursor_wraps_at_both_ends() {
		let principles = sample();
		let mut app = App::new(&principles, &[]);
		assert_eq!(app.available_state.selected(), Some(0));

		// Up from the top wraps to the bottom.
		update(&mut app, key(KeyCode::Up));
		assert_eq!(app.available_state.selected(), Some(2));

		// Down from the bottom wraps to the top.
		update(&mut app, key(KeyCode::Down));
		assert_eq!(app.available_state.selected(), Some(0));
	}

	#[test]
	fn reorder_does_nothing_on_the_available_pane() {
		let principles = sample();
		let mut app = App::new(&principles, &[]);
		assert_eq!(app.focus, Pane::Available);
		update(&mut app, key(KeyCode::Char('J')));
		assert_eq!(app.available, vec![0, 1, 2]);
	}

	#[test]
	fn confirm_and_abort_report_to_the_loop() {
		let principles = sample();
		let mut app = App::new(&principles, &[1]);
		assert_eq!(update(&mut app, key(KeyCode::Enter)), Step::Confirm);
		assert_eq!(app.included, vec![1]);
		assert_eq!(update(&mut app, key(KeyCode::Esc)), Step::Abort);
	}
}
