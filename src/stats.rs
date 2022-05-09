use std::io::Seek;
use std::io::SeekFrom::Current;

use byteorder::{ReadBytesExt, LE};
use serde::Serialize;
use winapi::um::winnt::HANDLE;

use crate::mem_reader::{read_n_bytes, read_uptr32_t};

#[derive(Clone, Default, Debug, Serialize, PartialEq)]
pub struct LevelDeaths {
    pub level_1: f64,
    pub level_2: f64,
    pub level_3: f64,
    pub level_4: f64,
    pub level_5: f64,
    pub level_6: f64,
    pub level_7: f64,
    pub level_8: f64,
    pub level_9: f64,
    pub level_10: f64,
    pub level_11: f64,
    pub level_12: f64,
    pub level_13: f64,
    pub level_14: f64,
    pub level_15: f64,
    pub level_16: f64,
}

#[derive(Clone, Default, Debug, Serialize, PartialEq)]
pub struct EnemyDeaths {
    pub bat: f64,
    pub snake: f64,
    pub spider: f64,
    pub giant_spider: f64,
    pub caveman: f64,
    pub skeleton: f64,
    pub zombie: f64,
    pub vampire: f64,
    pub frog: f64,
    pub fire_frog: f64,
    pub mantrap: f64,
    pub piranha: f64,
    pub megamouth: f64,
    pub yeti: f64,
    pub yeti_king: f64,
    pub alien: f64,
    pub ufo: f64,
    pub alien_boss: f64,
    pub hawkman: f64,
    pub shopkeeper: f64,
    pub tomb_lord: f64,
    pub magma_man: f64,
    pub olmec: f64,
    pub ghost: f64,
}

#[derive(Clone, Default, Debug, Serialize, PartialEq)]
pub struct MiscDeaths {
    pub rock: f64,
    pub explosion: f64,
    pub crushed: f64,
    pub long_fall: f64,
    pub spikes: f64,
    pub boulder: f64,
    pub arrow_trap: f64,
    pub spear_trap: f64,
    pub smash_trap: f64,
    pub ceiling_trap: f64,
    pub pit: f64,
    pub lava: f64,
}

#[derive(Clone, Default, Debug, Serialize, PartialEq)]
pub struct EnemyKills {
    pub bat: f64,
    pub snake: f64,
    pub spider: f64,
    pub giant_spider: f64,
    pub caveman: f64,
    pub skeleton: f64,
    pub zombie: f64,
    pub vampire: f64,
    pub frog: f64,
    pub monkey: f64,
    pub fire_frog: f64,
    pub mantrap: f64,
    pub piranha: f64,
    pub megamouth: f64,
    pub yeti: f64,
    pub yeti_king: f64,
    pub alien: f64,
    pub ufo: f64,
    pub alien_boss: f64,
    pub hawkman: f64,
    pub shopkeeper: f64,
    pub tomb_lord: f64,
    pub olmec: f64,
}

#[derive(Clone, Default, Debug, Serialize, PartialEq)]
pub struct Stats {
    pub crates_opened: f64,
    pub chests_opened: f64,
    pub idols_grabbed: f64,
    pub idols_converted: f64,
    pub damsels_grabbed: f64,
    pub kisses_bought: f64,
    pub damsels_bought: f64,
    pub damsels_saved: f64,
    pub damsels_killed: f64,
    pub items_bought: f64,
    pub items_stolen: f64,
    pub dice_games_played: f64,
    pub dice_games_won: f64,
    pub dice_games_lost: f64,

    pub level_deaths: LevelDeaths,
    pub enemy_deaths: EnemyDeaths,
    pub misc_deaths: MiscDeaths,
    pub enemy_kills: EnemyKills,
}

impl Stats {
    pub fn pprint(&self) {
        println!("Crates Opened:     {}", self.crates_opened);
        println!("Chests Opened:     {}", self.chests_opened);

        println!("Idols Grabbed:     {}", self.idols_grabbed);
        println!("Idols Converted:   {}", self.idols_converted);

        println!("Damsels Grabbed:   {}", self.damsels_grabbed);
        println!("Kisses Bought:     {}", self.kisses_bought);
        println!("Damsels Bought:    {}", self.damsels_bought);
        println!("Damsels Saved:     {}", self.damsels_saved);
        println!("Damsels Killed:    {}", self.damsels_killed);

        println!("Items Bought:      {}", self.items_bought);
        println!("Items Stolen:      {}", self.items_stolen);

        println!("Dice Games Played: {}", self.dice_games_played);
        println!("Dice Games Won:    {}", self.dice_games_won);
        println!("Dice Games Lost:   {}", self.dice_games_lost);

        println!("");
        println!("Levels:");
        println!(
            "  Mines:     {:5} {:5} {:5} {:5}",
            self.level_deaths.level_1,
            self.level_deaths.level_2,
            self.level_deaths.level_3,
            self.level_deaths.level_4
        );
        println!(
            "  Jungle:    {:5} {:5} {:5} {:5}",
            self.level_deaths.level_5,
            self.level_deaths.level_6,
            self.level_deaths.level_7,
            self.level_deaths.level_8
        );
        println!(
            "  Ice Caves: {:5} {:5} {:5} {:5}",
            self.level_deaths.level_9,
            self.level_deaths.level_10,
            self.level_deaths.level_11,
            self.level_deaths.level_12
        );
        println!(
            "  Temple:    {:5} {:5} {:5} {:5}",
            self.level_deaths.level_13,
            self.level_deaths.level_14,
            self.level_deaths.level_15,
            self.level_deaths.level_16
        );

        println!("");
        println!("Misc:");
        println!("  Rock:         {}", self.misc_deaths.rock);
        println!("  Explosion:    {}", self.misc_deaths.explosion);
        println!("  Crushed:      {}", self.misc_deaths.crushed);
        println!("  Long Fall:    {}", self.misc_deaths.long_fall);
        println!("  Spikes:       {}", self.misc_deaths.spikes);
        println!("  Boulder:      {}", self.misc_deaths.boulder);
        println!("  Arrow Trap:   {}", self.misc_deaths.arrow_trap);
        println!("  Spear Trap:   {}", self.misc_deaths.spear_trap);
        println!("  Smash Trap:   {}", self.misc_deaths.smash_trap);
        println!("  Ceiling Trap: {}", self.misc_deaths.ceiling_trap);
        println!("  Pit:          {}", self.misc_deaths.pit);
        println!("  Lava:         {}", self.misc_deaths.lava);

        println!("");
        println!("Enemies:       Deaths   Kills");
        println!(
            "  Bat:          {:5}   {}",
            self.enemy_deaths.bat, self.enemy_kills.bat
        );
        println!(
            "  Snake:        {:5}   {}",
            self.enemy_deaths.snake, self.enemy_kills.snake
        );
        println!(
            "  Spider:       {:5}   {}",
            self.enemy_deaths.spider, self.enemy_kills.spider
        );
        println!(
            "  Giant Spider: {:5}   {}",
            self.enemy_deaths.giant_spider, self.enemy_kills.giant_spider
        );
        println!(
            "  Caveman:      {:5}   {}",
            self.enemy_deaths.caveman, self.enemy_kills.caveman
        );
        println!(
            "  Skeleton:     {:5}   {}",
            self.enemy_deaths.skeleton, self.enemy_kills.skeleton
        );
        println!(
            "  Zombie:       {:5}   {}",
            self.enemy_deaths.zombie, self.enemy_kills.zombie
        );
        println!(
            "  Vampire:      {:5}   {}",
            self.enemy_deaths.vampire, self.enemy_kills.vampire
        );
        println!(
            "  Frog:         {:5}   {}",
            self.enemy_deaths.frog, self.enemy_kills.frog
        );
        println!("  Monkey:       {:5}   {}", "", self.enemy_kills.monkey);
        println!(
            "  Fire_frog:    {:5}   {}",
            self.enemy_deaths.fire_frog, self.enemy_kills.fire_frog
        );
        println!(
            "  Mantrap:      {:5}   {}",
            self.enemy_deaths.mantrap, self.enemy_kills.mantrap
        );
        println!(
            "  Piranha:      {:5}   {}",
            self.enemy_deaths.piranha, self.enemy_kills.piranha
        );
        println!(
            "  Megamouth:    {:5}   {}",
            self.enemy_deaths.megamouth, self.enemy_kills.megamouth
        );
        println!(
            "  Yeti:         {:5}   {}",
            self.enemy_deaths.yeti, self.enemy_kills.yeti
        );
        println!(
            "  Yeti King:    {:5}   {}",
            self.enemy_deaths.yeti_king, self.enemy_kills.yeti_king
        );
        println!(
            "  Alien:        {:5}   {}",
            self.enemy_deaths.alien, self.enemy_kills.alien
        );
        println!(
            "  Ufo:          {:5}   {}",
            self.enemy_deaths.ufo, self.enemy_kills.ufo
        );
        println!(
            "  Alien_boss:   {:5}   {}",
            self.enemy_deaths.alien_boss, self.enemy_kills.alien_boss
        );
        println!(
            "  Hawkman:      {:5}   {}",
            self.enemy_deaths.hawkman, self.enemy_kills.hawkman
        );
        println!(
            "  Shopkeeper:   {:5}   {}",
            self.enemy_deaths.shopkeeper, self.enemy_kills.shopkeeper
        );
        println!(
            "  Tomb Lord:    {:5}   {}",
            self.enemy_deaths.tomb_lord, self.enemy_kills.tomb_lord
        );
        println!("  Magma Man:    {:5}", self.enemy_deaths.magma_man);
        println!(
            "  Olmec:        {:5}   {}",
            self.enemy_deaths.olmec, self.enemy_kills.olmec
        );
        println!("  Ghost:        {:5}", self.enemy_deaths.ghost);
    }

    pub fn from_memory(process: HANDLE, base_addr: usize) -> anyhow::Result<Self> {
        let mut stats = Self::default();
        stats.update_from_memory(process, base_addr)?;
        Ok(stats)
    }

    pub fn update_from_memory(&mut self, process: HANDLE, base_addr: usize) -> anyhow::Result<()> {
        let start = base_addr + 0x0018f124;
        let offset = read_uptr32_t(process, start)? as usize;
        let offset = read_uptr32_t(process, offset + 0x4)? as usize;
        let offset = offset + 0xd58;

        let mut cursor = read_n_bytes(process, offset, 0x2d8)?;

        self.crates_opened = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.chests_opened = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.idols_grabbed = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.idols_converted = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.damsels_grabbed = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.kisses_bought = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.damsels_bought = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.damsels_saved = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.damsels_killed = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.items_bought = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.items_stolen = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.dice_games_played = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.dice_games_won = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        self.dice_games_lost = cursor.read_f64::<LE>()?;
        cursor.seek(Current(8 * 4))?;

        // Level Deaths
        self.level_deaths.level_1 = cursor.read_f64::<LE>()?;

        let level_deaths_list_offset =
            read_uptr32_t(process, (offset as u64 + cursor.position() + 0x8) as usize)? as usize;
        let level_deaths_list_offset = read_uptr32_t(process, level_deaths_list_offset)? as usize;
        let mut level_deaths_cursor =
            read_n_bytes(process, level_deaths_list_offset + 0x20, 0x158)?;

        self.level_deaths.level_2 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_3 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_4 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_5 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_6 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_7 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_8 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_9 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_10 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_11 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_12 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_13 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_14 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_15 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        self.level_deaths.level_16 = level_deaths_cursor.read_f64::<LE>()?;
        level_deaths_cursor.seek(Current(0x10))?;

        cursor.seek(Current(8 * 4))?;

        // Enemy Deaths
        self.enemy_deaths.bat = cursor.read_f64::<LE>()?;
        let enemy_deaths_list_offset =
            read_uptr32_t(process, (offset as u64 + cursor.position() + 0x8) as usize)? as usize;
        let enemy_deaths_list_offset = read_uptr32_t(process, enemy_deaths_list_offset)? as usize;
        let mut enemy_deaths_cursor =
            read_n_bytes(process, enemy_deaths_list_offset + 0x20, 0x218)?;

        self.enemy_deaths.snake = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.spider = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.giant_spider = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.caveman = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.skeleton = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.zombie = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.vampire = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.frog = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.fire_frog = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.mantrap = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.piranha = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.megamouth = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.yeti = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.yeti_king = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.alien = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.ufo = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.alien_boss = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.hawkman = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.shopkeeper = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.tomb_lord = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.magma_man = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.olmec = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        self.enemy_deaths.ghost = enemy_deaths_cursor.read_f64::<LE>()?;
        enemy_deaths_cursor.seek(Current(0x10))?;

        cursor.seek(Current(8 * 4))?;

        // Misc Deaths
        self.misc_deaths.rock = cursor.read_f64::<LE>()?;
        let misc_deaths_list_offset =
            read_uptr32_t(process, (offset as u64 + cursor.position() + 0x8) as usize)? as usize;
        let misc_deaths_list_offset = read_uptr32_t(process, misc_deaths_list_offset)? as usize;
        let mut misc_deaths_cursor = read_n_bytes(process, misc_deaths_list_offset + 0x20, 0xf8)?;

        self.misc_deaths.explosion = misc_deaths_cursor.read_f64::<LE>()?;
        misc_deaths_cursor.seek(Current(0x10))?;

        self.misc_deaths.crushed = misc_deaths_cursor.read_f64::<LE>()?;
        misc_deaths_cursor.seek(Current(0x10))?;

        self.misc_deaths.long_fall = misc_deaths_cursor.read_f64::<LE>()?;
        misc_deaths_cursor.seek(Current(0x10))?;

        self.misc_deaths.spikes = misc_deaths_cursor.read_f64::<LE>()?;
        misc_deaths_cursor.seek(Current(0x10))?;

        self.misc_deaths.boulder = misc_deaths_cursor.read_f64::<LE>()?;
        misc_deaths_cursor.seek(Current(0x10))?;

        self.misc_deaths.arrow_trap = misc_deaths_cursor.read_f64::<LE>()?;
        misc_deaths_cursor.seek(Current(0x10))?;

        self.misc_deaths.spear_trap = misc_deaths_cursor.read_f64::<LE>()?;
        misc_deaths_cursor.seek(Current(0x10))?;

        self.misc_deaths.smash_trap = misc_deaths_cursor.read_f64::<LE>()?;
        misc_deaths_cursor.seek(Current(0x10))?;

        self.misc_deaths.ceiling_trap = misc_deaths_cursor.read_f64::<LE>()?;
        misc_deaths_cursor.seek(Current(0x10))?;

        self.misc_deaths.pit = misc_deaths_cursor.read_f64::<LE>()?;
        misc_deaths_cursor.seek(Current(0x10))?;

        self.misc_deaths.lava = misc_deaths_cursor.read_f64::<LE>()?;
        misc_deaths_cursor.seek(Current(0x10))?;

        cursor.seek(Current(8 * 4))?;

        // Enemy Kills
        self.enemy_kills.bat = cursor.read_f64::<LE>()?;
        let enemy_kills_list_offset =
            read_uptr32_t(process, (offset as u64 + cursor.position() + 0x8) as usize)? as usize;
        let enemy_kills_list_offset = read_uptr32_t(process, enemy_kills_list_offset)? as usize;
        let mut enemy_kills_cursor = read_n_bytes(process, enemy_kills_list_offset + 0x20, 0x1e8)?;

        self.enemy_kills.snake = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.spider = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.giant_spider = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.caveman = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.skeleton = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.zombie = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.vampire = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.frog = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.fire_frog = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.mantrap = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.piranha = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.megamouth = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.yeti = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.yeti_king = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.alien = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.ufo = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.alien_boss = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.hawkman = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.shopkeeper = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.tomb_lord = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        self.enemy_kills.olmec = enemy_kills_cursor.read_f64::<LE>()?;
        enemy_kills_cursor.seek(Current(0x10))?;

        cursor.seek(Current(8 * 4))?;

        self.enemy_kills.monkey = cursor.read_f64::<LE>()?;

        Ok(())
    }
}
