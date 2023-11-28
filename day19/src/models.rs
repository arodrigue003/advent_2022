#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Blueprint {
    pub index: usize,
    pub ore: usize,               // ore
    pub clay: usize,              // ore
    pub obsidian: (usize, usize), // ore, clay
    pub geode: (usize, usize),    // ore, obsidian
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Robots {
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
    pub geode: usize,
}

impl Robots {
    pub fn new() -> Self {
        Self {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Resources {
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
    pub geode: usize,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ForbiddenBuilds {
    pub ore: bool,
    pub clay: bool,
    pub obsidian: bool,
    pub geode: bool,
}

impl ForbiddenBuilds {
    pub fn new() -> Self {
        Self {
            ore: false,
            clay: false,
            obsidian: false,
            geode: false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Game {
    pub robots: Robots,
    pub resources: Resources,
    pub forbidden_builds: ForbiddenBuilds,
}

impl Game {
    pub fn new() -> Self {
        Self {
            robots: Robots::new(),
            resources: Resources::new(),
            forbidden_builds: ForbiddenBuilds::new(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BuildOption {
    None,
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MaxTime {
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
    pub geode: usize,
}

impl MaxTime {
    pub fn new(time: usize, blueprint: &Blueprint) -> Self {
        Self {
            ore: time - blueprint.ore - 2 - 2,
            clay: time - 6,
            obsidian: time - 4,
            geode: time - 2,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MaxRobot {
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
}

impl MaxRobot {
    pub fn new(blueprint: &Blueprint) -> Self {
        Self {
            ore: blueprint.ore.max(
                blueprint
                    .clay
                    .max(blueprint.obsidian.0.max(blueprint.geode.0)),
            ),
            clay: blueprint.obsidian.1,
            obsidian: blueprint.geode.1,
        }
    }
}
