use firecore_world::serialized::SerializedNPCType;

pub fn load_npc_types(npc_type_dir: &str) -> crate::ResultT<Vec<SerializedNPCType>> {
    let mut types = Vec::new();

    for entry in std::fs::read_dir(npc_type_dir)?.map(|entry| entry.unwrap()) {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            let data_path = path.join(name.clone() + ".ron");
            let sprite_path = path.join(name.clone() + ".png");
            let battle_sprite_path = path.join("battle.png");
            let bytes =  std::fs::read(sprite_path)?;
            let data = ron::from_str(&std::fs::read_to_string(data_path)?)?;
            println!("Added NPC type {}!", &name);
            let mut npc_type = SerializedNPCType {
                identifier: name,
                data,
                sprite: bytes,
                battle_sprite: None,
            };
            if let Ok(bytes) = std::fs::read(battle_sprite_path) {
                npc_type.battle_sprite = Some(bytes);
            }
            types.push(npc_type);
        }
    }

    Ok(types)
}