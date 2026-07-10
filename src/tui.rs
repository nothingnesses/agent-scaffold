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
			Clear,
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

/// A button in the save-confirmation modal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Button {
	Save,
	Cancel,
}

impl Button {
	/// The other button (there are only two).
	fn other(self) -> Self {
		match self {
			Button::Save => Button::Cancel,
			Button::Cancel => Button::Save,
		}
	}
}

/// Which interaction mode the selector is in. Exactly one holds at a time, so
/// states like "confirming and filtering at once" or "a focused button while
/// not confirming" are unrepresentable. `Filtering` means keystrokes edit the
/// `App::filter` query; the applied filter itself lives on `App` because its
/// narrowing of the Available pane persists back in `Editing`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
	Editing,
	Filtering,
	Confirming { button: Button },
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

/// A restorable snapshot of the editable state, for undo and redo.
#[derive(Clone)]
struct Snapshot {
	included: Vec<usize>,
	available: Vec<usize>,
	focus: Pane,
	available_selected: Option<usize>,
	included_selected: Option<usize>,
}

/// The whole selector state.
///
/// `included` (ordered) and `available` are disjoint and together cover every
/// principle index exactly once. The editing mutations are `toggle` (move one
/// index between the two) and `reorder` (swap adjacent within `included`), so
/// the invariant holds by construction; undo and redo restore whole-state
/// snapshots, which are themselves valid.
struct App<'a> {
	principles: &'a [Principle],
	/// Included principle indices, in the user's chosen order.
	included: Vec<usize>,
	/// Available (not included) indices, seeded in `default_order`.
	available: Vec<usize>,
	focus: Pane,
	available_state: ListState,
	included_state: ListState,
	/// Pre-edit snapshots for undo (most recent last).
	undo_stack: Vec<Snapshot>,
	/// Snapshots undone and available to redo (most recent last).
	redo_stack: Vec<Snapshot>,
	/// The current interaction mode (editing, filtering, or the save modal).
	mode: Mode,
	/// The active Available-pane filter (case-insensitive substring over name,
	/// id, and tags). Empty means no filter. Narrows only the Available pane.
	filter: String,
	/// Caller-supplied lines describing what saving will do, shown in the modal.
	save_summary: Vec<String>,
}

impl<'a> App<'a> {
	fn new(
		principles: &'a [Principle],
		initial_included: &[usize],
		save_summary: Vec<String>,
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
			undo_stack: Vec::new(),
			redo_stack: Vec::new(),
			mode: Mode::Editing,
			filter: String::new(),
			save_summary,
		}
	}

	/// Whether principle `idx` matches the active filter (always true when the
	/// filter is empty). Case-insensitive substring over name, id, and tags.
	fn matches_filter(
		&self,
		idx: usize,
	) -> bool {
		if self.filter.is_empty() {
			return true;
		}
		let needle = self.filter.to_lowercase();
		let p = &self.principles[idx];
		p.name.to_lowercase().contains(&needle)
			|| p.id.to_lowercase().contains(&needle)
			|| p.tags.iter().any(|t| t.to_lowercase().contains(&needle))
	}

	/// Positions into `self.available` that match the filter, in `available`
	/// order. With an empty filter this is `0 .. available.len()`. This is the
	/// projection the Available cursor and toggle map through.
	fn available_visible(&self) -> Vec<usize> {
		self.available
			.iter()
			.enumerate()
			.filter(|(_, &idx)| self.matches_filter(idx))
			.map(|(pos, _)| pos)
			.collect()
	}

	/// Keep the Available cursor within its visible (filtered) length.
	fn clamp_available(&mut self) {
		let len = self.available_visible().len();
		match len {
			0 => self.available_state.select(None),
			_ => {
				let clamped = self.available_state.selected().unwrap_or(0).min(len - 1);
				self.available_state.select(Some(clamped));
			}
		}
	}

	/// Capture the current editable state.
	fn snapshot(&self) -> Snapshot {
		Snapshot {
			included: self.included.clone(),
			available: self.available.clone(),
			focus: self.focus,
			available_selected: self.available_state.selected(),
			included_selected: self.included_state.selected(),
		}
	}

	/// Restore a captured snapshot. The active filter is view state and is left
	/// as-is, so the restored Available selection is re-clamped to the current
	/// visible length.
	fn restore(
		&mut self,
		snap: Snapshot,
	) {
		self.included = snap.included;
		self.available = snap.available;
		self.focus = snap.focus;
		self.available_state.select(snap.available_selected);
		self.included_state.select(snap.included_selected);
		self.clamp_available();
	}

	/// Record the pre-edit state so the edit can be undone, and drop the redo
	/// history (a fresh edit forks it).
	fn checkpoint(&mut self) {
		self.undo_stack.push(self.snapshot());
		self.redo_stack.clear();
	}

	/// Undo the last edit, moving it onto the redo stack.
	fn undo(&mut self) {
		if let Some(prev) = self.undo_stack.pop() {
			let current = self.snapshot();
			self.redo_stack.push(current);
			self.restore(prev);
		}
	}

	/// Redo the last undone edit, moving it back onto the undo stack.
	fn redo(&mut self) {
		if let Some(next) = self.redo_stack.pop() {
			let current = self.snapshot();
			self.undo_stack.push(current);
			self.restore(next);
		}
	}

	fn focused_len(&self) -> usize {
		match self.focus {
			Pane::Available => self.available_visible().len(),
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

	/// Enter filter mode, focusing Available (the only filtered pane).
	fn enter_filter(&mut self) {
		self.focus = Pane::Available;
		self.mode = Mode::Filtering;
		self.clamp_available();
	}

	/// Move the highlighted principle to the other pane. `insert` decides where
	/// it lands relative to that pane's cursor (before or after), and leaves the
	/// destination cursor on the moved item.
	///
	/// The Available cursor is an index into the filtered projection, so moving
	/// out of Available maps that visible index back to the underlying position.
	/// When a filter is active, an item returned to Available is appended (the
	/// Available pool order is not user-meaningful, and "before/after cursor" is
	/// ambiguous under a projection); with no filter, behaviour is unchanged.
	fn toggle_with(
		&mut self,
		insert: fn(&mut Vec<usize>, &mut ListState, usize),
	) {
		match self.focus {
			Pane::Available => {
				let visible = self.available_visible();
				let Some(sel) = self.available_state.selected() else {
					return;
				};
				let Some(&ap) = visible.get(sel) else {
					return;
				};
				self.checkpoint();
				let idx = self.available.remove(ap);
				insert(&mut self.included, &mut self.included_state, idx);
				self.clamp_available();
			}
			Pane::Included => {
				let Some(pos) = self.included_state.selected() else {
					return;
				};
				if pos >= self.included.len() {
					return;
				}
				self.checkpoint();
				let idx = self.included.remove(pos);
				if self.filter.is_empty() {
					insert(&mut self.available, &mut self.available_state, idx);
				} else {
					self.available.push(idx);
					self.clamp_available();
				}
				self.clamp_focused();
			}
		}
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
		self.checkpoint();
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
		self.checkpoint();
		self.included.swap(pos, pos + 1);
		self.included_state.select(Some(pos + 1));
	}

	/// The underlying principle indices currently shown in the Available pane
	/// (its filtered projection), in display order.
	fn available_display(&self) -> Vec<usize> {
		self.available_visible().iter().map(|&ap| self.available[ap]).collect()
	}

	/// The principle currently under the focused cursor, if any. The Available
	/// cursor indexes the filtered projection.
	fn highlighted(&self) -> Option<&Principle> {
		let idx = match self.focus {
			Pane::Available => *self.available_display().get(self.available_state.selected()?)?,
			Pane::Included => *self.included.get(self.included_state.selected()?)?,
		};
		Some(&self.principles[idx])
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

	/// The body text of the save-confirmation modal: what saving will do.
	fn confirm_message(&self) -> Text<'static> {
		let mut lines =
			vec![Line::from(format!("{} principle(s) will be included.", self.included.len()))];
		for line in &self.save_summary {
			lines.push(Line::from(line.clone()));
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

	// While the save modal is open it is modal: only button navigation and
	// activate/cancel are handled; editing keys are ignored.
	if let Mode::Confirming {
		button,
	} = app.mode
	{
		match key.code {
			KeyCode::Left
			| KeyCode::Right
			| KeyCode::Tab
			| KeyCode::BackTab
			| KeyCode::Char('h')
			| KeyCode::Char('l') =>
				app.mode = Mode::Confirming {
					button: button.other(),
				},
			KeyCode::Enter => match button {
				Button::Save => return Step::Confirm,
				Button::Cancel => app.mode = Mode::Editing,
			},
			KeyCode::Esc => app.mode = Mode::Editing,
			_ => {}
		}
		return Step::Continue;
	}

	// While filtering, keystrokes edit the query and the Available pane narrows
	// live; Enter applies (keeping the filter), Esc clears it.
	if app.mode == Mode::Filtering {
		match key.code {
			KeyCode::Enter => app.mode = Mode::Editing,
			KeyCode::Esc => {
				app.filter.clear();
				app.mode = Mode::Editing;
				app.clamp_available();
			}
			KeyCode::Backspace => {
				app.filter.pop();
				app.clamp_available();
			}
			KeyCode::Char(c) => {
				app.filter.push(c);
				app.clamp_available();
			}
			_ => {}
		}
		return Step::Continue;
	}

	match key.code {
		KeyCode::Char('q') | KeyCode::Esc => return Step::Abort,
		KeyCode::Enter =>
		// Open the save modal; focus Cancel by default so an accidental confirm never writes.
			app.mode = Mode::Confirming {
				button: Button::Cancel,
			},
		KeyCode::Char('i') => app.toggle_with(insert_before_cursor),
		KeyCode::Char('a') => app.toggle_with(insert_after_cursor),
		KeyCode::Char('/') => app.enter_filter(),
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
		KeyCode::Char('u') => app.undo(),
		KeyCode::Char('U') => app.redo(),
		KeyCode::Up | KeyCode::Char('k') => app.cursor_up(),
		KeyCode::Down | KeyCode::Char('j') => app.cursor_down(),
		_ => {}
	}
	Step::Continue
}

/// The fixed presentation of a pane (its varying-per-pane, non-state fields).
struct PaneSpec {
	title: String,
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
	let block = Block::bordered().title(title).border_style(border);
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
	let available_display = app.available_display();
	let available_title = if app.filter.is_empty() {
		format!("Available ({})", available_display.len())
	} else {
		format!("Available ({})  /{}", available_display.len(), app.filter)
	};
	let included_title = format!("Included ({})", app.included.len());

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
		&available_display,
		&mut app.available_state,
		PaneSpec {
			title: available_title,
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
			title: included_title,
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

	let help = if app.mode == Mode::Filtering {
		Paragraph::new(format!("filter> {}    enter: apply   esc: clear", app.filter))
	} else {
		Paragraph::new(
			"i/a insert  J/K reorder  u/U undo/redo  / filter  tab/h/l focus  j/k cursor  enter save  q abort",
		)
		.centered()
		.dim()
	};
	frame.render_widget(help, help_area);

	if let Mode::Confirming {
		button,
	} = app.mode
	{
		render_confirm_modal(frame, app, button);
	}
}

/// A centred rectangle `width` x `height` clamped to `area`.
fn centered_rect(
	width: u16,
	height: u16,
	area: Rect,
) -> Rect {
	let width = width.min(area.width);
	let height = height.min(area.height);
	Rect {
		x: area.x + (area.width - width) / 2,
		y: area.y + (area.height - height) / 2,
		width,
		height,
	}
}

/// Render one modal button, highlighted when focused.
fn render_button(
	frame: &mut Frame,
	area: Rect,
	label: &str,
	focused: bool,
) {
	let (border, text) = if focused {
		(
			Style::default().fg(Color::Yellow),
			Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD),
		)
	} else {
		(Style::default().fg(Color::DarkGray), Style::default())
	};
	frame.render_widget(
		Paragraph::new(label).centered().style(text).block(Block::bordered().border_style(border)),
		area,
	);
}

/// Render the save-confirmation modal over the selector.
fn render_confirm_modal(
	frame: &mut Frame,
	app: &App,
	button: Button,
) {
	let message = app.confirm_message();
	// message lines + a blank + the button row (3) + the outer border (2).
	let height = message.lines.len() as u16 + 1 + 3 + 2;
	let area = centered_rect(60, height, frame.area());

	frame.render_widget(Clear, area);
	let block = Block::bordered().title("Save?");
	let inner = block.inner(area);
	frame.render_widget(block, area);

	let [message_area, buttons_area] =
		Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(inner);
	frame.render_widget(
		Paragraph::new(message).wrap(Wrap {
			trim: true,
		}),
		message_area,
	);

	let [save_area, cancel_area] =
		Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
			.areas(buttons_area);
	render_button(frame, save_area, "Save", button == Button::Save);
	render_button(frame, cancel_area, "Cancel", button == Button::Cancel);
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
	save_summary: Vec<String>,
) -> io::Result<Option<Vec<usize>>> {
	let mut app = App::new(principles, initial_included, save_summary);
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
/// in the available pane ordered by `default_order`. `save_summary` is shown in
/// the save-confirmation modal to describe what saving will do.
pub fn run_selection(
	principles: &[Principle],
	initial_included: &[usize],
	save_summary: Vec<String>,
) -> io::Result<Option<Vec<usize>>> {
	let mut terminal = ratatui::init();
	let outcome = run(&mut terminal, principles, initial_included, save_summary);
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
		let mut app = App::new(&principles, &[], Vec::new());
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
		let mut app = App::new(&principles, &[1, 2], Vec::new());
		assert_eq!(app.available, vec![0]);

		// The destination (Included) cursor sits on its second row; `i` moves
		// the item just before it, and the cursor follows to the moved item.
		app.included_state.select(Some(1));
		update(&mut app, key(KeyCode::Char('i')));
		assert_eq!(app.included, vec![1, 0, 2]);
		assert_eq!(app.included_state.selected(), Some(1));
	}

	#[test]
	fn undo_and_redo_an_edit() {
		let principles = sample();
		let mut app = App::new(&principles, &[], Vec::new());

		// Include 'a', then undo back to the empty selection, then redo.
		update(&mut app, key(KeyCode::Char('i')));
		assert_eq!(app.included, vec![0]);
		assert_eq!(app.available, vec![1, 2]);

		update(&mut app, key(KeyCode::Char('u')));
		assert!(app.included.is_empty());
		assert_eq!(app.available, vec![0, 1, 2]);

		update(&mut app, key(KeyCode::Char('U')));
		assert_eq!(app.included, vec![0]);
		assert_eq!(app.available, vec![1, 2]);
	}

	#[test]
	fn a_fresh_edit_forks_the_redo_history() {
		let principles = sample();
		let mut app = App::new(&principles, &[], Vec::new());
		update(&mut app, key(KeyCode::Char('i'))); // include 'a'
		update(&mut app, key(KeyCode::Char('u'))); // undo; the toggle is now redoable

		update(&mut app, key(KeyCode::Char('i'))); // a new edit clears the redo stack
		assert!(app.redo_stack.is_empty());
		update(&mut app, key(KeyCode::Char('U'))); // redo is now a no-op
		assert_eq!(app.included, vec![0]);
	}

	#[test]
	fn undo_with_no_history_is_a_no_op() {
		let principles = sample();
		let mut app = App::new(&principles, &[], Vec::new());
		update(&mut app, key(KeyCode::Char('u')));
		assert_eq!(app.available, vec![0, 1, 2]);
		assert!(app.included.is_empty());
	}

	#[test]
	fn a_inserts_after_the_destination_cursor() {
		let principles = sample();
		let mut app = App::new(&principles, &[1, 2], Vec::new());
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
		let mut app = App::new(&principles, &[0, 1, 2], Vec::new());
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
		let mut app = App::new(&principles, &[], Vec::new());
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
		let mut app = App::new(&principles, &[], Vec::new());
		assert_eq!(app.focus, Pane::Available);
		update(&mut app, key(KeyCode::Char('J')));
		assert_eq!(app.available, vec![0, 1, 2]);
	}

	#[test]
	fn abort_reports_to_the_loop() {
		let principles = sample();
		let mut app = App::new(&principles, &[1], Vec::new());
		// From editing, Esc aborts the whole selector.
		assert_eq!(update(&mut app, key(KeyCode::Esc)), Step::Abort);
	}

	#[test]
	fn enter_opens_the_modal_which_saves_or_cancels() {
		let principles = sample();
		let mut app = App::new(&principles, &[1], Vec::new());

		// Enter opens the modal (does not save yet) and focuses Cancel.
		assert_eq!(update(&mut app, key(KeyCode::Enter)), Step::Continue);
		assert_eq!(
			app.mode,
			Mode::Confirming {
				button: Button::Cancel
			}
		);

		// Enter on Cancel closes the modal without saving.
		assert_eq!(update(&mut app, key(KeyCode::Enter)), Step::Continue);
		assert_eq!(app.mode, Mode::Editing);

		// Reopen, move focus to Save, then Enter saves.
		update(&mut app, key(KeyCode::Enter));
		update(&mut app, key(KeyCode::Right));
		assert_eq!(
			app.mode,
			Mode::Confirming {
				button: Button::Save
			}
		);
		assert_eq!(update(&mut app, key(KeyCode::Enter)), Step::Confirm);
		assert_eq!(app.included, vec![1]);
	}

	#[test]
	fn modal_ignores_editing_keys_and_esc_cancels() {
		let principles = sample();
		let mut app = App::new(&principles, &[1], Vec::new());
		update(&mut app, key(KeyCode::Enter)); // open modal
		let before = app.included.clone();
		// An editing key does nothing while the modal is open.
		update(&mut app, key(KeyCode::Char('i')));
		assert_eq!(app.included, before);
		assert!(matches!(app.mode, Mode::Confirming { .. }));
		// Esc cancels the modal.
		update(&mut app, key(KeyCode::Esc));
		assert_eq!(app.mode, Mode::Editing);
	}

	// -- Step 5c: the interactive Available filter. `sample()` names/ids are
	// "a"/"b"/"c" and every principle carries the tag "t", so a query like "b"
	// matches by name/id and "t" matches by tag.

	#[test]
	fn filter_narrows_the_available_projection_and_persists() {
		let principles = sample();
		let mut app = App::new(&principles, &[], Vec::new());

		update(&mut app, key(KeyCode::Char('/')));
		assert_eq!(app.mode, Mode::Filtering);
		update(&mut app, key(KeyCode::Char('b')));
		assert_eq!(app.available_display(), vec![1]);
		assert_eq!(app.available_state.selected(), Some(0));

		// Enter applies and keeps the filter back in Editing.
		update(&mut app, key(KeyCode::Enter));
		assert_eq!(app.mode, Mode::Editing);
		assert_eq!(app.filter, "b");
		assert_eq!(app.available_display(), vec![1]);
	}

	#[test]
	fn filter_matches_tags_not_only_names() {
		let principles = sample();
		let mut app = App::new(&principles, &[], Vec::new());
		update(&mut app, key(KeyCode::Char('/')));
		update(&mut app, key(KeyCode::Char('t'))); // tag on all; no name contains "t"
		assert_eq!(app.available_display(), vec![0, 1, 2]);
	}

	#[test]
	fn editing_keys_type_into_the_filter_while_filtering() {
		let principles = sample();
		let mut app = App::new(&principles, &[], Vec::new());
		update(&mut app, key(KeyCode::Char('/')));
		update(&mut app, key(KeyCode::Char('i'))); // 'i' is query text here, not a toggle
		assert_eq!(app.filter, "i");
		assert!(app.included.is_empty());
	}

	#[test]
	fn backspace_edits_and_esc_clears_the_filter() {
		let principles = sample();
		let mut app = App::new(&principles, &[], Vec::new());
		update(&mut app, key(KeyCode::Char('/')));
		update(&mut app, key(KeyCode::Char('b')));
		update(&mut app, key(KeyCode::Char('x'))); // "bx" matches nothing
		assert!(app.available_display().is_empty());

		update(&mut app, key(KeyCode::Backspace)); // back to "b"
		assert_eq!(app.filter, "b");
		assert_eq!(app.available_display(), vec![1]);

		update(&mut app, key(KeyCode::Esc)); // clear
		assert_eq!(app.mode, Mode::Editing);
		assert_eq!(app.filter, "");
		assert_eq!(app.available_display(), vec![0, 1, 2]);
	}

	#[test]
	fn toggle_under_filter_moves_the_matched_principle() {
		let principles = sample();
		let mut app = App::new(&principles, &[], Vec::new());
		update(&mut app, key(KeyCode::Char('/')));
		update(&mut app, key(KeyCode::Char('c'))); // only c (idx 2) matches
		update(&mut app, key(KeyCode::Enter));
		assert_eq!(app.available_display(), vec![2]);

		// Include the highlighted match; the correct principle moves.
		update(&mut app, key(KeyCode::Char('i')));
		assert_eq!(app.included, vec![2]);
		assert_eq!(app.available, vec![0, 1]);
		// With the filter still "c", nothing remains visible; the cursor clears.
		assert!(app.available_display().is_empty());
		assert_eq!(app.available_state.selected(), None);
		assert_eq!(app.included.len() + app.available.len(), principles.len());
	}
}
