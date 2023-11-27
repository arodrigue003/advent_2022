use crate::models::{Blueprint, BuildOption, ForbiddenBuilds, Game, MaxTime, Resources, Robots};

impl Game {
    pub fn build_options(
        &mut self,
        current_time: usize,
        max_time: &MaxTime,
        blueprint: &Blueprint,
    ) -> [Option<BuildOption>; 5] {
        [
            Some(BuildOption::None),
            if current_time <= max_time.ore
                && !self.forbidden_builds.ore
                && self.resources.ore >= blueprint.ore
            {
                self.forbidden_builds.ore = true;
                Some(BuildOption::Ore)
            } else {
                None
            },
            if current_time <= max_time.clay
                && !self.forbidden_builds.clay
                && self.resources.ore >= blueprint.clay
            {
                self.forbidden_builds.clay = true;
                Some(BuildOption::Clay)
            } else {
                None
            },
            if current_time <= max_time.obsidian
                && !self.forbidden_builds.obsidian
                && self.resources.ore >= blueprint.obsidian.0
                && self.resources.clay >= blueprint.obsidian.1
            {
                self.forbidden_builds.obsidian = true;
                Some(BuildOption::Obsidian)
            } else {
                None
            },
            if current_time <= max_time.geode
                && !self.forbidden_builds.geode
                && self.resources.ore >= blueprint.geode.0
                && self.resources.obsidian >= blueprint.geode.1
            {
                self.forbidden_builds.geode = true;
                Some(BuildOption::Geode)
            } else {
                None
            },
        ]
    }

    pub fn tick_with_build_option(&mut self, build_option: BuildOption, blueprint: &Blueprint) {
        // Update resources
        self.resources.ore += self.robots.ore;
        self.resources.clay += self.robots.clay;
        self.resources.obsidian += self.robots.obsidian;
        self.resources.geode += self.robots.geode;

        // Update robot count depending of the build option
        match build_option {
            BuildOption::None => {}
            BuildOption::Ore => {
                self.resources.ore -= blueprint.ore;
                self.robots.ore += 1
            }
            BuildOption::Clay => {
                self.resources.ore -= blueprint.clay;
                self.robots.clay += 1
            }
            BuildOption::Obsidian => {
                self.resources.ore -= blueprint.obsidian.0;
                self.resources.clay -= blueprint.obsidian.1;
                self.robots.obsidian += 1
            }
            BuildOption::Geode => {
                self.resources.ore -= blueprint.geode.0;
                self.resources.obsidian -= blueprint.geode.1;
                self.robots.geode += 1
            }
        }

        // Reset forbidden builds if we built something
        if build_option != BuildOption::None {
            self.forbidden_builds.ore = false;
            self.forbidden_builds.clay = false;
            self.forbidden_builds.obsidian = false;
            self.forbidden_builds.geode = false;
        }
    }

    pub fn tick(&self, build_option: BuildOption, blueprint: &Blueprint) -> Option<Self> {
        match build_option {
            BuildOption::None => Some(Self {
                robots: Robots {
                    ore: self.robots.ore,
                    clay: self.robots.clay,
                    obsidian: self.robots.obsidian,
                    geode: self.robots.geode,
                },
                resources: Resources {
                    ore: self.resources.ore + self.robots.ore,
                    clay: self.resources.clay + self.robots.clay,
                    obsidian: self.resources.obsidian + self.robots.obsidian,
                    geode: self.resources.geode + self.robots.geode,
                },
                forbidden_builds: ForbiddenBuilds {
                    ore: self.forbidden_builds.ore,
                    clay: self.forbidden_builds.clay,
                    obsidian: self.forbidden_builds.obsidian,
                    geode: self.forbidden_builds.geode,
                },
            }),
            BuildOption::Ore => {
                if !self.forbidden_builds.ore && self.resources.ore >= blueprint.ore {
                    Some(Self {
                        robots: Robots {
                            ore: self.robots.ore + 1,
                            clay: self.robots.clay,
                            obsidian: self.robots.obsidian,
                            geode: self.robots.geode,
                        },
                        resources: Resources {
                            ore: self.resources.ore + self.robots.ore - blueprint.ore,
                            clay: self.resources.clay + self.robots.clay,
                            obsidian: self.resources.obsidian + self.robots.obsidian,
                            geode: self.resources.geode + self.robots.geode,
                        },
                        forbidden_builds: ForbiddenBuilds {
                            ore: false,
                            clay: false,
                            obsidian: false,
                            geode: false,
                        },
                    })
                } else {
                    None
                }
            }
            BuildOption::Clay => {
                if !self.forbidden_builds.clay && self.resources.ore >= blueprint.clay {
                    Some(Self {
                        robots: Robots {
                            ore: self.robots.ore,
                            clay: self.robots.clay + 1,
                            obsidian: self.robots.obsidian,
                            geode: self.robots.geode,
                        },
                        resources: Resources {
                            ore: self.resources.ore + self.robots.ore - blueprint.clay,
                            clay: self.resources.clay + self.robots.clay,
                            obsidian: self.resources.obsidian + self.robots.obsidian,
                            geode: self.resources.geode + self.robots.geode,
                        },
                        forbidden_builds: ForbiddenBuilds {
                            ore: false,
                            clay: false,
                            obsidian: false,
                            geode: false,
                        },
                    })
                } else {
                    None
                }
            }
            BuildOption::Obsidian => {
                if !self.forbidden_builds.obsidian
                    && self.resources.ore >= blueprint.obsidian.0
                    && self.resources.clay >= blueprint.obsidian.1
                {
                    Some(Self {
                        robots: Robots {
                            ore: self.robots.ore,
                            clay: self.robots.clay,
                            obsidian: self.robots.obsidian + 1,
                            geode: self.robots.geode,
                        },
                        resources: Resources {
                            ore: self.resources.ore + self.robots.ore - blueprint.obsidian.0,
                            clay: self.resources.clay + self.robots.clay - blueprint.obsidian.1,
                            obsidian: self.resources.obsidian + self.robots.obsidian,
                            geode: self.resources.geode + self.robots.geode,
                        },
                        forbidden_builds: ForbiddenBuilds {
                            ore: false,
                            clay: false,
                            obsidian: false,
                            geode: false,
                        },
                    })
                } else {
                    None
                }
            }
            BuildOption::Geode => {
                if !self.forbidden_builds.geode
                    && self.resources.ore >= blueprint.geode.0
                    && self.resources.obsidian >= blueprint.geode.1
                {
                    Some(Self {
                        robots: Robots {
                            ore: self.robots.ore,
                            clay: self.robots.clay,
                            obsidian: self.robots.obsidian,
                            geode: self.robots.geode + 1,
                        },
                        resources: Resources {
                            ore: self.resources.ore + self.robots.ore - blueprint.geode.0,
                            clay: self.resources.clay + self.robots.clay,
                            obsidian: self.resources.obsidian + self.robots.obsidian
                                - blueprint.geode.1,
                            geode: self.resources.geode + self.robots.geode,
                        },
                        forbidden_builds: ForbiddenBuilds {
                            ore: false,
                            clay: false,
                            obsidian: false,
                            geode: false,
                        },
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn tick_heuristic(&self, build_option: BuildOption, blueprint: &Blueprint) -> Option<Self> {
        match build_option {
            BuildOption::None => Some(Self {
                robots: Robots {
                    ore: self.robots.ore,
                    clay: self.robots.clay,
                    obsidian: self.robots.obsidian,
                    geode: self.robots.geode,
                },
                resources: Resources {
                    ore: self.resources.ore + self.robots.ore,
                    clay: self.resources.clay + self.robots.clay,
                    obsidian: self.resources.obsidian + self.robots.obsidian,
                    geode: self.resources.geode + self.robots.geode,
                },
                forbidden_builds: ForbiddenBuilds {
                    ore: self.forbidden_builds.ore,
                    clay: self.forbidden_builds.clay,
                    obsidian: self.forbidden_builds.obsidian,
                    geode: self.forbidden_builds.geode,
                },
            }),
            BuildOption::Ore => {
                if !self.forbidden_builds.ore && self.resources.ore >= blueprint.ore {
                    Some(Self {
                        robots: Robots {
                            ore: self.robots.ore + 1,
                            clay: self.robots.clay,
                            obsidian: self.robots.obsidian,
                            geode: self.robots.geode,
                        },
                        resources: Resources {
                            ore: self.resources.ore + self.robots.ore - blueprint.ore,
                            clay: self.resources.clay + self.robots.clay,
                            obsidian: self.resources.obsidian + self.robots.obsidian,
                            geode: self.resources.geode + self.robots.geode,
                        },
                        forbidden_builds: ForbiddenBuilds {
                            ore: false,
                            clay: false,
                            obsidian: false,
                            geode: false,
                        },
                    })
                } else {
                    None
                }
            }
            BuildOption::Clay => {
                if !self.forbidden_builds.clay && self.resources.ore >= blueprint.clay {
                    Some(Self {
                        robots: Robots {
                            ore: self.robots.ore,
                            clay: self.robots.clay + 1,
                            obsidian: self.robots.obsidian,
                            geode: self.robots.geode,
                        },
                        resources: Resources {
                            ore: self.resources.ore + self.robots.ore - blueprint.clay,
                            clay: self.resources.clay + self.robots.clay,
                            obsidian: self.resources.obsidian + self.robots.obsidian,
                            geode: self.resources.geode + self.robots.geode,
                        },
                        forbidden_builds: ForbiddenBuilds {
                            ore: false,
                            clay: false,
                            obsidian: false,
                            geode: false,
                        },
                    })
                } else {
                    None
                }
            }
            BuildOption::Obsidian => {
                if !self.forbidden_builds.obsidian
                    && self.resources.ore >= blueprint.obsidian.0
                    && self.resources.clay >= blueprint.obsidian.1
                {
                    Some(Self {
                        robots: Robots {
                            ore: self.robots.ore,
                            clay: self.robots.clay,
                            obsidian: self.robots.obsidian + 1,
                            geode: self.robots.geode,
                        },
                        resources: Resources {
                            ore: self.resources.ore + self.robots.ore - blueprint.obsidian.0,
                            clay: self.resources.clay + self.robots.clay - blueprint.obsidian.1,
                            obsidian: self.resources.obsidian + self.robots.obsidian,
                            geode: self.resources.geode + self.robots.geode,
                        },
                        forbidden_builds: ForbiddenBuilds {
                            ore: true,
                            clay: true,
                            obsidian: false,
                            geode: false,
                        },
                    })
                } else {
                    None
                }
            }
            BuildOption::Geode => {
                if !self.forbidden_builds.geode
                    && self.resources.ore >= blueprint.geode.0
                    && self.resources.obsidian >= blueprint.geode.1
                {
                    Some(Self {
                        robots: Robots {
                            ore: self.robots.ore,
                            clay: self.robots.clay,
                            obsidian: self.robots.obsidian,
                            geode: self.robots.geode + 1,
                        },
                        resources: Resources {
                            ore: self.resources.ore + self.robots.ore - blueprint.geode.0,
                            clay: self.resources.clay + self.robots.clay,
                            obsidian: self.resources.obsidian + self.robots.obsidian
                                - blueprint.geode.1,
                            geode: self.resources.geode + self.robots.geode,
                        },
                        forbidden_builds: ForbiddenBuilds {
                            ore: true,
                            clay: true,
                            obsidian: false,
                            geode: false,
                        },
                    })
                } else {
                    None
                }
            }
        }
    }
}

fn simulate_game_rec(
    game: &Game,
    current_time: usize,
    time: usize,
    blueprint: &Blueprint,
) -> usize {
    if current_time == time {
        return game.resources.geode * blueprint.index;
    }

    // Get the game for every build option
    let ore = game.tick(BuildOption::Ore, blueprint);
    let clay = game.tick(BuildOption::Clay, blueprint);
    let obsidian = game.tick(BuildOption::Obsidian, blueprint);
    let geode = game.tick(BuildOption::Geode, blueprint);

    // Update the none option depending on what was built
    let mut none = game.tick(BuildOption::None, blueprint);

    if let Some(none) = none.as_mut() {
        none.forbidden_builds.ore = none.forbidden_builds.ore || ore.is_some();
        none.forbidden_builds.clay = none.forbidden_builds.clay || clay.is_some();
        none.forbidden_builds.obsidian = none.forbidden_builds.obsidian || obsidian.is_some();
        none.forbidden_builds.geode = none.forbidden_builds.geode || geode.is_some();
    }

    [ore, clay, obsidian, geode, none]
        .into_iter()
        .filter_map(|game| {
            game.map(|game| simulate_game_rec(&game, current_time + 1, time, blueprint))
        })
        .max()
        .unwrap()
}

pub fn simulate_game(time: usize, blueprint: &Blueprint) -> usize {
    let game = Game::new();
    simulate_game_rec(&game, 0, time, blueprint)
}

fn simulate_game_with_scout_rec(
    game: &mut Game,
    current_time: usize,
    time: usize,
    max_time: &MaxTime,
    blueprint: &Blueprint,
) -> usize {
    if current_time == time {
        return game.resources.geode * blueprint.index;
    }

    // Get build option
    let build_options = game.build_options(current_time, max_time, blueprint);

    // Apply them recursively
    build_options
        .into_iter()
        .flatten()
        .map(|build_option| {
            let mut game = game.clone();
            game.tick_with_build_option(build_option, blueprint);
            simulate_game_with_scout_rec(&mut game, current_time + 1, time, max_time, blueprint)
        })
        .max()
        .unwrap()
}

pub fn simulate_game_with_scout(time: usize, blueprint: &Blueprint) -> usize {
    let max_time = MaxTime::new(time, blueprint);
    let mut game = Game::new();
    simulate_game_with_scout_rec(&mut game, 0, time, &max_time, blueprint)
}

fn simulate_game_rec_heuristic(
    game: &Game,
    current_time: usize,
    time: usize,
    blueprint: &Blueprint,
) -> usize {
    if current_time == time {
        return game.resources.geode;
    }

    // Get the game for every build option
    let ore = game.tick_heuristic(BuildOption::Ore, blueprint);
    let clay = game.tick_heuristic(BuildOption::Clay, blueprint);
    let obsidian = game.tick_heuristic(BuildOption::Obsidian, blueprint);
    let geode = game.tick_heuristic(BuildOption::Geode, blueprint);

    // Update the none option depending on what was built
    let mut none = game.tick_heuristic(BuildOption::None, blueprint);

    if let Some(none) = none.as_mut() {
        none.forbidden_builds.ore = none.forbidden_builds.ore || ore.is_some();
        none.forbidden_builds.clay = none.forbidden_builds.clay || clay.is_some();
        none.forbidden_builds.obsidian = none.forbidden_builds.obsidian || obsidian.is_some();
        none.forbidden_builds.geode = none.forbidden_builds.geode || geode.is_some();
    }

    [ore, clay, obsidian, geode, none]
        .into_iter()
        .filter_map(|game| {
            game.map(|game| simulate_game_rec_heuristic(&game, current_time + 1, time, blueprint))
        })
        .max()
        .unwrap()
}

pub fn simulate_game_heuristic(time: usize, blueprint: &Blueprint) -> usize {
    let game = Game::new();
    simulate_game_rec_heuristic(&game, 0, time, blueprint)
}
