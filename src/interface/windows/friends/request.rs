use derive_new::new;
use procedural::*;

use crate::input::UserEvent;
use crate::interface::*;
use crate::network::Friend;

#[derive(new)]
pub struct FriendRequestWindow {
    friend: Friend,
}

impl FriendRequestWindow {
    pub const WINDOW_CLASS: &str = "friend_request";
}

impl PrototypeWindow for FriendRequestWindow {
    fn to_window(&self, window_cache: &WindowCache, interface_settings: &InterfaceSettings, available_space: Size) -> Window {
        let elements = vec![
            Text::default()
                .with_text(format!("^ffaa00{}^000000 wants to be friends with you", self.friend.name))
                .wrap(),
            Button::default()
                .with_text("reject")
                .with_event(UserEvent::RejectFriendRequest {
                    account_id: self.friend.account_id,
                    character_id: self.friend.character_id,
                })
                .with_width(dimension!(50%))
                .wrap(),
            Button::default()
                .with_text("accept")
                .with_event(UserEvent::AcceptFriendRequest {
                    account_id: self.friend.account_id,
                    character_id: self.friend.character_id,
                })
                .with_width(dimension!(!))
                .wrap(),
        ];

        WindowBuilder::default()
            .with_title("Friend request".to_string())
            // NOTE: we give the builder a class but we don't implement the `window_class` method
            // of the trait. This way we can open multiple windos of this type but we can still
            // close them with the class name.
            .with_class(Self::WINDOW_CLASS.to_owned())
            .with_size(constraint!(250, ?))
            .with_elements(elements)
            .build(window_cache, interface_settings, available_space)
    }
}
