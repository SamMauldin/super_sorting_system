pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3 {
    fn format_spaced(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }
}

pub struct Sign {
    pub text: [String; 4],
    pub location: Vec3,
}

impl Sign {
    fn to_command(&self) -> String {
        fn escape_line(s: &str) -> String {
            format!(r#""\"{}\"""#, s)
        }

        format!(
            r#"setblock {} oak_sign{{Text1:{},Text2:{},Text3:{},Text4:{}}}"#,
            self.location.format_spaced(),
            escape_line(&self.text[0]),
            escape_line(&self.text[1]),
            escape_line(&self.text[2]),
            escape_line(&self.text[3])
        )
    }
}

pub struct Barrel {
    pub location: Vec3,
    pub items: Vec<Item>,
}

impl Barrel {
    fn to_command(&self) -> String {
        format!(
            r#"setblock {} barrel{{Items:[{}]}}"#,
            self.location.format_spaced(),
            self.items
                .iter()
                .map(|item| item.to_nbt())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

pub struct Item {
    pub slot: i64,
    pub name: String,
    pub count: i64,
}

impl Item {
    fn to_nbt(&self) -> String {
        format!(
            r#"{{id:{},Slot:{},Count:{}}}"#,
            self.name, self.slot, self.count
        )
    }
}

pub struct Scenario {
    pub barrels: Vec<Barrel>,
    pub signs: Vec<Sign>,
}

impl Scenario {
    pub fn to_command(&self) -> Option<String> {
        let mut barrel_commands = self
            .barrels
            .iter()
            .map(|barrel| barrel.to_command())
            .collect::<Vec<String>>();
        let mut sign_commands = self
            .signs
            .iter()
            .map(|sign| sign.to_command())
            .collect::<Vec<String>>();

        let mut commands = vec![];
        commands.append(&mut barrel_commands);
        commands.append(&mut sign_commands);
        commands.push(String::from(""));

        let command = commands.join("\n");

        if command.len() == 0 {
            None
        } else {
            Some(command)
        }
    }
}
