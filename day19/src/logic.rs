use crate::models::{Blueprint, BuildOption, Game, MaxTime};
use rayon::prelude::*;

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

    pub fn tick(&mut self, build_option: BuildOption, blueprint: &Blueprint) {
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
}

fn simulate_game_with_scout_rec(
    game: &mut Game,
    current_time: usize,
    time: usize,
    max_time: &MaxTime,
    blueprint: &Blueprint,
) -> usize {
    if current_time == time {
        return game.resources.geode;
    }

    // Get build option
    let build_options = game.build_options(current_time, max_time, blueprint);

    // Apply them recursively
    if current_time <= 10 {
        build_options
            .into_par_iter()
            .flatten()
            .map(|build_option| {
                let mut game = game.clone();
                game.tick(build_option, blueprint);
                simulate_game_with_scout_rec(&mut game, current_time + 1, time, max_time, blueprint)
            })
            .max()
            .unwrap()
    } else {
        build_options
            .into_iter()
            .flatten()
            .map(|build_option| {
                let mut game = game.clone();
                game.tick(build_option, blueprint);
                simulate_game_with_scout_rec(&mut game, current_time + 1, time, max_time, blueprint)
            })
            .max()
            .unwrap()
    }
}

pub fn simulate_game_with_scout(time: usize, blueprint: &Blueprint) -> usize {
    let max_time = MaxTime::new(time, blueprint);
    let mut game = Game::new();
    simulate_game_with_scout_rec(&mut game, 0, time, &max_time, blueprint)
}
