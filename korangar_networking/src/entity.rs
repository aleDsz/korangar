use ragnarok_packets::*;

#[derive(Debug)]
pub struct EntityData {
    pub entity_id: EntityId,
    pub movement_speed: u16,
    pub job: u16,
    pub head: u16,
    pub weapon: u16,
    pub head_bottom: u16,
    pub head_top: u16,
    pub head_middle: u16,
    pub hair_color: u16,
    pub clothes_color: u16,
    pub robe: u16,
    pub position: WorldPosition,
    pub destination: Option<WorldPosition>,
    pub health_points: i32,
    pub maximum_health_points: i32,
    pub head_direction: usize,
    pub sex: Sex,
}

impl EntityData {
    pub fn from_character(account_id: AccountId, character_information: CharacterInformation, position: WorldPosition) -> Self {
        println!("character_information: {:?}", character_information);

        Self {
            entity_id: EntityId(account_id.0),
            movement_speed: character_information.movement_speed as u16,
            job: character_information.job as u16,
            head: character_information.head as u16,
            weapon: character_information.weapon as u16,
            head_bottom: character_information.accessory as u16,
            head_top: character_information.accessory2 as u16,
            head_middle: character_information.accessory3 as u16,
            hair_color: character_information.hair_color as u16,
            clothes_color: character_information.body_palette as u16,
            robe: character_information.robe_palette as u16,
            position,
            destination: None,
            health_points: character_information.health_points as i32,
            maximum_health_points: character_information.maximum_health_points as i32,
            head_direction: 0, // TODO: get correct rotation
            sex: character_information.sex,
        }
    }
}

impl From<EntityAppearedPacket> for EntityData {
    fn from(packet: EntityAppearedPacket) -> Self {
        Self {
            entity_id: packet.entity_id,
            movement_speed: packet.movement_speed,
            job: packet.job,
            head: packet.head,
            weapon: packet.weapon as u16,
            head_bottom: packet.accessory,
            head_top: packet.accessory2,
            head_middle: packet.accessory3,
            hair_color: packet.head_palette,
            clothes_color: packet.body_palette,
            robe: packet.robe,
            position: packet.position,
            destination: None,
            health_points: packet.health_points,
            maximum_health_points: packet.maximum_health_points,
            head_direction: packet.head_direction as usize,
            sex: packet.sex,
        }
    }
}

impl From<EntityAppeared2Packet> for EntityData {
    fn from(packet: EntityAppeared2Packet) -> Self {
        Self {
            entity_id: packet.entity_id,
            movement_speed: packet.movement_speed,
            job: packet.job,
            head: packet.head,
            weapon: packet.weapon as u16,
            head_bottom: packet.accessory,
            head_top: packet.accessory2,
            head_middle: packet.accessory3,
            hair_color: packet.head_palette,
            clothes_color: packet.body_palette,
            robe: packet.robe,
            position: packet.position,
            destination: None,
            health_points: packet.health_points,
            maximum_health_points: packet.maximum_health_points,
            head_direction: packet.head_direction as usize,
            sex: packet.sex,
        }
    }
}

impl From<MovingEntityAppearedPacket> for EntityData {
    fn from(packet: MovingEntityAppearedPacket) -> Self {
        let (origin, destination) = packet.position.to_origin_destination();

        Self {
            entity_id: packet.entity_id,
            movement_speed: packet.movement_speed,
            job: packet.job,
            head: packet.head,
            weapon: packet.weapon as u16,
            head_bottom: packet.accessory,
            head_top: packet.accessory2,
            head_middle: packet.accessory3,
            hair_color: packet.head_palette,
            clothes_color: packet.body_palette,
            robe: packet.robe,
            position: origin,
            destination: Some(destination),
            health_points: packet.health_points,
            maximum_health_points: packet.maximum_health_points,
            head_direction: packet.head_direction as usize,
            sex: packet.sex,
        }
    }
}
