use cgmath::Array;
use procedural::*;

use crate::graphics::{InterfaceRenderer, Renderer};
use crate::input::{HotbarSlot, MouseInputMode};
use crate::interface::*;
use crate::inventory::Skill;

pub struct HotbarContainer {
    skills: Remote<[Option<Skill>; 10]>,
    weak_self: Option<WeakElementCell>,
    state: ContainerState,
}

impl HotbarContainer {
    pub fn new(skills: Remote<[Option<Skill>; 10]>) -> Self {
        let elements = {
            let skills = skills.borrow();

            skills
                .iter()
                .cloned()
                .enumerate()
                .map(|(slot, skill)| {
                    let skill_source = SkillSource::Hotbar { slot: HotbarSlot(slot) };
                    let skill_box = SkillBox::new(
                        skill,
                        skill_source,
                        Box::new(move |mouse_mode| matches!(mouse_mode, MouseInputMode::MoveSkill(source, _) if *source != skill_source)),
                    );

                    skill_box.wrap()
                })
                .collect()
        };

        let weak_self = None;
        let state = ContainerState::new(elements);

        Self { skills, weak_self, state }
    }
}

impl Element for HotbarContainer {
    fn get_state(&self) -> &ElementState {
        &self.state.state
    }

    fn get_state_mut(&mut self) -> &mut ElementState {
        &mut self.state.state
    }

    fn link_back(&mut self, weak_self: WeakElementCell, weak_parent: Option<WeakElementCell>) {
        self.weak_self = Some(weak_self.clone());
        self.state.link_back(weak_self, weak_parent);
    }

    fn is_focusable(&self) -> bool {
        self.state.is_focusable::<false>()
    }

    fn focus_next(&self, self_cell: ElementCell, caller_cell: Option<ElementCell>, focus: Focus) -> Option<ElementCell> {
        self.state.focus_next::<false>(self_cell, caller_cell, focus)
    }

    fn restore_focus(&self, self_cell: ElementCell) -> Option<ElementCell> {
        self.state.restore_focus(self_cell)
    }

    fn resolve(&mut self, placement_resolver: &mut PlacementResolver, interface_settings: &InterfaceSettings, theme: &Theme) {
        let size_constraint = &constraint!(100%, ?);
        self.state.resolve(
            placement_resolver,
            interface_settings,
            theme,
            size_constraint,
            Vector2::from_value(3.0),
        );
    }

    fn update(&mut self) -> Option<ChangeEvent> {
        if self.skills.consume_changed() {
            let weak_parent = self.state.state.parent_element.take();
            let weak_self = self.weak_self.take().unwrap();

            *self = Self::new(self.skills.clone());
            // important: link back after creating elements, otherwise focus navigation and
            // scrolling would break
            self.link_back(weak_self, weak_parent);

            return Some(ChangeEvent::Reresolve); // TODO: ReresolveWindow
        }

        None
    }

    fn hovered_element(&self, mouse_position: Position, mouse_mode: &MouseInputMode) -> HoverInformation {
        match mouse_mode {
            MouseInputMode::MoveSkill(..) | MouseInputMode::None => self.state.hovered_element(mouse_position, mouse_mode, false),
            _ => HoverInformation::Missed,
        }
    }

    fn render(
        &self,
        render_target: &mut <InterfaceRenderer as Renderer>::Target,
        renderer: &InterfaceRenderer,
        state_provider: &StateProvider,
        interface_settings: &InterfaceSettings,
        theme: &Theme,
        parent_position: Position,
        clip_size: ClipSize,
        hovered_element: Option<&dyn Element>,
        focused_element: Option<&dyn Element>,
        mouse_mode: &MouseInputMode,
        second_theme: bool,
    ) {
        let mut renderer = self
            .state
            .state
            .element_renderer(render_target, renderer, interface_settings, parent_position, clip_size);

        self.state.render(
            &mut renderer,
            state_provider,
            interface_settings,
            theme,
            hovered_element,
            focused_element,
            mouse_mode,
            second_theme,
        );
    }
}
