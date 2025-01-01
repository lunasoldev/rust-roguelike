
pub struct Player {
    pub pos: [u16; 2],

    hp: u16,
    max_hp: u16,

    xp: u32,
    xp_needed: u32,
    lvl: u16,

    str: u16,
    dex: u16,

    atk: u16,
    def: u16,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: [0, 0],

            hp: 10,
            max_hp: 10,

            xp: 0,
            xp_needed: 10,
            lvl: 1,

            str: 5,
            dex: 5,

            atk: 5,
            def: 5,
        }
    }

    pub fn hp(&mut self) -> u16 {
        self.hp
    }

    pub fn max_hp(&mut self) -> u16 {
        self.max_hp
    }

    pub fn str(&mut self) -> u16 {
        self.str
    }

    pub fn dex(&mut self) -> u16 {
        self.dex
    }

    pub fn atk(&mut self) -> u16 {
        self.atk
    }

    pub fn def(&mut self) -> u16 {
        self.def
    }

    pub fn lvl(&mut self) -> u16 {
        self.lvl
    }

    pub fn xp(&mut self) -> u32 {
        self.xp
    }

    pub fn xp_needed(&mut self) -> u32 {
        self.xp_needed
    }

    pub fn move_x(&mut self, amount: i16, bound: u16) {
        let new_pos = self.pos[0] as i16 + amount;

        self.pos[0] = new_pos
            .clamp(0, bound as i16)
            as u16;
    }

    pub fn move_y(&mut self, amount: i16, bound: u16) {
        let new_pos = self.pos[1] as i16 + amount;

        self.pos[1] = new_pos
            .clamp(0, bound as i16)
            as u16;
    }
}
