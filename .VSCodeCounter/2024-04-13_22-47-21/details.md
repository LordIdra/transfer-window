# Details

Date : 2024-04-13 22:47:21

Directory /home/idra/GitHub/transfer-window

Total : 163 files,  10312 codes, 242 comments, 1483 blanks, all 12037 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [Cargo.lock](/Cargo.lock) | TOML | 3,606 | 2 | 410 | 4,018 |
| [Cargo.toml](/Cargo.toml) | TOML | 37 | 2 | 4 | 43 |
| [common/Cargo.toml](/common/Cargo.toml) | TOML | 12 | 0 | 2 | 14 |
| [common/src/lib.rs](/common/src/lib.rs) | Rust | 1 | 0 | 0 | 1 |
| [common/src/numerical_methods.rs](/common/src/numerical_methods.rs) | Rust | 7 | 4 | 0 | 11 |
| [common/src/numerical_methods/bisection.rs](/common/src/numerical_methods/bisection.rs) | Rust | 38 | 3 | 4 | 45 |
| [common/src/numerical_methods/closest_ellipse_point.rs](/common/src/numerical_methods/closest_ellipse_point.rs) | Rust | 25 | 5 | 11 | 41 |
| [common/src/numerical_methods/halley.rs](/common/src/numerical_methods/halley.rs) | Rust | 60 | 0 | 8 | 68 |
| [common/src/numerical_methods/itp.rs](/common/src/numerical_methods/itp.rs) | Rust | 13 | 2 | 2 | 17 |
| [common/src/numerical_methods/laguerre.rs](/common/src/numerical_methods/laguerre.rs) | Rust | 66 | 1 | 6 | 73 |
| [common/src/numerical_methods/newton_raphson.rs](/common/src/numerical_methods/newton_raphson.rs) | Rust | 58 | 0 | 6 | 64 |
| [common/src/numerical_methods/util.rs](/common/src/numerical_methods/util.rs) | Rust | 33 | 3 | 11 | 47 |
| [controller/Cargo.toml](/controller/Cargo.toml) | TOML | 19 | 0 | 4 | 23 |
| [controller/src/event_handler.rs](/controller/src/event_handler.rs) | Rust | 99 | 3 | 21 | 123 |
| [controller/src/main.rs](/controller/src/main.rs) | Rust | 82 | 0 | 19 | 101 |
| [model/Cargo.toml](/model/Cargo.toml) | TOML | 20 | 0 | 3 | 23 |
| [model/resources/prediction-test-cases/collision-with-moon/encounters.json](/model/resources/prediction-test-cases/collision-with-moon/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/collision-with-moon/metadata.json](/model/resources/prediction-test-cases/collision-with-moon/metadata.json) | JSON | 6 | 0 | 1 | 7 |
| [model/resources/prediction-test-cases/collision-with-moon/objects.json](/model/resources/prediction-test-cases/collision-with-moon/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/encounter-with-earth-and-moon/encounters.json](/model/resources/prediction-test-cases/encounter-with-earth-and-moon/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/encounter-with-earth-and-moon/metadata.json](/model/resources/prediction-test-cases/encounter-with-earth-and-moon/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/encounter-with-earth-and-moon/objects.json](/model/resources/prediction-test-cases/encounter-with-earth-and-moon/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/encounter-with-earth/encounters.json](/model/resources/prediction-test-cases/encounter-with-earth/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/encounter-with-earth/metadata.json](/model/resources/prediction-test-cases/encounter-with-earth/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/encounter-with-earth/objects.json](/model/resources/prediction-test-cases/encounter-with-earth/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/escape-from-earth/encounters.json](/model/resources/prediction-test-cases/escape-from-earth/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/escape-from-earth/metadata.json](/model/resources/prediction-test-cases/escape-from-earth/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/escape-from-earth/objects.json](/model/resources/prediction-test-cases/escape-from-earth/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/escape-from-moon-1/encounters.json](/model/resources/prediction-test-cases/escape-from-moon-1/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/escape-from-moon-1/metadata.json](/model/resources/prediction-test-cases/escape-from-moon-1/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/escape-from-moon-1/objects.json](/model/resources/prediction-test-cases/escape-from-moon-1/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/escape-from-moon-2/encounters.json](/model/resources/prediction-test-cases/escape-from-moon-2/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/escape-from-moon-2/metadata.json](/model/resources/prediction-test-cases/escape-from-moon-2/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/escape-from-moon-2/objects.json](/model/resources/prediction-test-cases/escape-from-moon-2/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-1/encounters.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-1/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-1/metadata.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-1/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-1/objects.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-1/objects.json) | JSON | 35 | 0 | 0 | 35 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-2/encounters.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-2/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-2/metadata.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-2/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-2/objects.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-2/objects.json) | JSON | 35 | 0 | 0 | 35 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-3/encounters.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-3/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-3/metadata.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-3/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-3/objects.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-3/objects.json) | JSON | 35 | 0 | 0 | 35 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-4/encounters.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-4/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-4/metadata.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-4/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-4/objects.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-4/objects.json) | JSON | 35 | 0 | 0 | 35 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-5/encounters.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-5/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-5/metadata.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-5/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/hyperbolic-moon-encounter-5/objects.json](/model/resources/prediction-test-cases/hyperbolic-moon-encounter-5/objects.json) | JSON | 35 | 0 | 0 | 35 |
| [model/resources/prediction-test-cases/insanity-1/encounters.json](/model/resources/prediction-test-cases/insanity-1/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/insanity-1/metadata.json](/model/resources/prediction-test-cases/insanity-1/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/insanity-1/objects.json](/model/resources/prediction-test-cases/insanity-1/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/insanity-2/encounters.json](/model/resources/prediction-test-cases/insanity-2/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/insanity-2/metadata.json](/model/resources/prediction-test-cases/insanity-2/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/insanity-2/objects.json](/model/resources/prediction-test-cases/insanity-2/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/insanity-3/encounters.json](/model/resources/prediction-test-cases/insanity-3/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/insanity-3/metadata.json](/model/resources/prediction-test-cases/insanity-3/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/insanity-3/objects.json](/model/resources/prediction-test-cases/insanity-3/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/many-moon-encounters/encounters.json](/model/resources/prediction-test-cases/many-moon-encounters/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/many-moon-encounters/metadata.json](/model/resources/prediction-test-cases/many-moon-encounters/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/many-moon-encounters/objects.json](/model/resources/prediction-test-cases/many-moon-encounters/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/moon-slingshot-to-escape-earth/encounters.json](/model/resources/prediction-test-cases/moon-slingshot-to-escape-earth/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/moon-slingshot-to-escape-earth/metadata.json](/model/resources/prediction-test-cases/moon-slingshot-to-escape-earth/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/moon-slingshot-to-escape-earth/objects.json](/model/resources/prediction-test-cases/moon-slingshot-to-escape-earth/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/no-encounters/encounters.json](/model/resources/prediction-test-cases/no-encounters/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/no-encounters/metadata.json](/model/resources/prediction-test-cases/no-encounters/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/no-encounters/objects.json](/model/resources/prediction-test-cases/no-encounters/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/parallel-with-moon/encounters.json](/model/resources/prediction-test-cases/parallel-with-moon/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/parallel-with-moon/metadata.json](/model/resources/prediction-test-cases/parallel-with-moon/metadata.json) | JSON | 6 | 0 | 1 | 7 |
| [model/resources/prediction-test-cases/parallel-with-moon/objects.json](/model/resources/prediction-test-cases/parallel-with-moon/objects.json) | JSON | 28 | 0 | 0 | 28 |
| [model/resources/prediction-test-cases/two-moons-varied-encounters-1/encounters.json](/model/resources/prediction-test-cases/two-moons-varied-encounters-1/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/two-moons-varied-encounters-1/metadata.json](/model/resources/prediction-test-cases/two-moons-varied-encounters-1/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/two-moons-varied-encounters-1/objects.json](/model/resources/prediction-test-cases/two-moons-varied-encounters-1/objects.json) | JSON | 35 | 0 | 0 | 35 |
| [model/resources/prediction-test-cases/two-moons-varied-encounters-2/encounters.json](/model/resources/prediction-test-cases/two-moons-varied-encounters-2/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/two-moons-varied-encounters-2/metadata.json](/model/resources/prediction-test-cases/two-moons-varied-encounters-2/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/two-moons-varied-encounters-2/objects.json](/model/resources/prediction-test-cases/two-moons-varied-encounters-2/objects.json) | JSON | 35 | 0 | 0 | 35 |
| [model/resources/prediction-test-cases/two-moons-varied-encounters-3/encounters.json](/model/resources/prediction-test-cases/two-moons-varied-encounters-3/encounters.json) | JSON | 1 | 0 | 0 | 1 |
| [model/resources/prediction-test-cases/two-moons-varied-encounters-3/metadata.json](/model/resources/prediction-test-cases/two-moons-varied-encounters-3/metadata.json) | JSON | 6 | 0 | 0 | 6 |
| [model/resources/prediction-test-cases/two-moons-varied-encounters-3/objects.json](/model/resources/prediction-test-cases/two-moons-varied-encounters-3/objects.json) | JSON | 35 | 0 | 0 | 35 |
| [model/src/components.rs](/model/src/components.rs) | Rust | 14 | 0 | 1 | 15 |
| [model/src/components/mass_component.rs](/model/src/components/mass_component.rs) | Rust | 13 | 1 | 3 | 17 |
| [model/src/components/name_component.rs](/model/src/components/name_component.rs) | Rust | 13 | 0 | 3 | 16 |
| [model/src/components/orbitable_component.rs](/model/src/components/orbitable_component.rs) | Rust | 13 | 1 | 3 | 17 |
| [model/src/components/stationary_component.rs](/model/src/components/stationary_component.rs) | Rust | 14 | 1 | 3 | 18 |
| [model/src/components/trajectory_component.rs](/model/src/components/trajectory_component.rs) | Rust | 142 | 13 | 24 | 179 |
| [model/src/components/trajectory_component/brute_force_tester.rs](/model/src/components/trajectory_component/brute_force_tester.rs) | Rust | 76 | 0 | 15 | 91 |
| [model/src/components/trajectory_component/burn.rs](/model/src/components/trajectory_component/burn.rs) | Rust | 149 | 1 | 32 | 182 |
| [model/src/components/trajectory_component/burn/burn_point.rs](/model/src/components/trajectory_component/burn/burn_point.rs) | Rust | 33 | 0 | 7 | 40 |
| [model/src/components/trajectory_component/orbit.rs](/model/src/components/trajectory_component/orbit.rs) | Rust | 192 | 1 | 45 | 238 |
| [model/src/components/trajectory_component/orbit/conic.rs](/model/src/components/trajectory_component/orbit/conic.rs) | Rust | 112 | 2 | 20 | 134 |
| [model/src/components/trajectory_component/orbit/conic/ellipse.rs](/model/src/components/trajectory_component/orbit/conic/ellipse.rs) | Rust | 233 | 3 | 26 | 262 |
| [model/src/components/trajectory_component/orbit/conic/hyperbola.rs](/model/src/components/trajectory_component/orbit/conic/hyperbola.rs) | Rust | 309 | 1 | 34 | 344 |
| [model/src/components/trajectory_component/orbit/orbit_direction.rs](/model/src/components/trajectory_component/orbit/orbit_direction.rs) | Rust | 29 | 0 | 5 | 34 |
| [model/src/components/trajectory_component/orbit/orbit_point.rs](/model/src/components/trajectory_component/orbit/orbit_point.rs) | Rust | 50 | 0 | 10 | 60 |
| [model/src/components/trajectory_component/orbit/scary_math.rs](/model/src/components/trajectory_component/orbit/scary_math.rs) | Rust | 139 | 12 | 25 | 176 |
| [model/src/components/trajectory_component/segment.rs](/model/src/components/trajectory_component/segment.rs) | Rust | 101 | 6 | 18 | 125 |
| [model/src/components/vessel_component.rs](/model/src/components/vessel_component.rs) | Rust | 9 | 1 | 2 | 12 |
| [model/src/debug.rs](/model/src/debug.rs) | Rust | 13 | 0 | 3 | 16 |
| [model/src/lib.rs](/model/src/lib.rs) | Rust | 270 | 8 | 60 | 338 |
| [model/src/storage.rs](/model/src/storage.rs) | Rust | 3 | 0 | 0 | 3 |
| [model/src/storage/component_storage.rs](/model/src/storage/component_storage.rs) | Rust | 146 | 7 | 20 | 173 |
| [model/src/storage/entity_allocator.rs](/model/src/storage/entity_allocator.rs) | Rust | 72 | 2 | 13 | 87 |
| [model/src/storage/entity_builder.rs](/model/src/storage/entity_builder.rs) | Rust | 36 | 2 | 7 | 45 |
| [model/src/systems.rs](/model/src/systems.rs) | Rust | 5 | 0 | 0 | 5 |
| [model/src/systems/time.rs](/model/src/systems/time.rs) | Rust | 65 | 0 | 10 | 75 |
| [model/src/systems/trajectory_prediction.rs](/model/src/systems/trajectory_prediction.rs) | Rust | 4 | 0 | 0 | 4 |
| [model/src/systems/trajectory_prediction/encounter.rs](/model/src/systems/trajectory_prediction/encounter.rs) | Rust | 35 | 0 | 10 | 45 |
| [model/src/systems/trajectory_prediction/fast_solver.rs](/model/src/systems/trajectory_prediction/fast_solver.rs) | Rust | 150 | 4 | 36 | 190 |
| [model/src/systems/trajectory_prediction/fast_solver/bounding.rs](/model/src/systems/trajectory_prediction/fast_solver/bounding.rs) | Rust | 37 | 4 | 7 | 48 |
| [model/src/systems/trajectory_prediction/fast_solver/bounding/ellipse.rs](/model/src/systems/trajectory_prediction/fast_solver/bounding/ellipse.rs) | Rust | 141 | 16 | 19 | 176 |
| [model/src/systems/trajectory_prediction/fast_solver/bounding/hyperbola.rs](/model/src/systems/trajectory_prediction/fast_solver/bounding/hyperbola.rs) | Rust | 81 | 1 | 15 | 97 |
| [model/src/systems/trajectory_prediction/fast_solver/bounding/sdf.rs](/model/src/systems/trajectory_prediction/fast_solver/bounding/sdf.rs) | Rust | 31 | 3 | 5 | 39 |
| [model/src/systems/trajectory_prediction/fast_solver/bounding/util.rs](/model/src/systems/trajectory_prediction/fast_solver/bounding/util.rs) | Rust | 59 | 9 | 10 | 78 |
| [model/src/systems/trajectory_prediction/fast_solver/bounding/window.rs](/model/src/systems/trajectory_prediction/fast_solver/bounding/window.rs) | Rust | 44 | 2 | 11 | 57 |
| [model/src/systems/trajectory_prediction/fast_solver/solver.rs](/model/src/systems/trajectory_prediction/fast_solver/solver.rs) | Rust | 59 | 15 | 14 | 88 |
| [model/src/systems/trajectory_prediction/fast_solver/solver/entrance_solver.rs](/model/src/systems/trajectory_prediction/fast_solver/solver/entrance_solver.rs) | Rust | 26 | 12 | 2 | 40 |
| [model/src/systems/trajectory_prediction/fast_solver/solver/exit_solver.rs](/model/src/systems/trajectory_prediction/fast_solver/solver/exit_solver.rs) | Rust | 78 | 28 | 15 | 121 |
| [model/src/systems/trajectory_prediction/old/brute_force_solver.rs](/model/src/systems/trajectory_prediction/old/brute_force_solver.rs) | Rust | 203 | 7 | 39 | 249 |
| [model/src/systems/trajectory_prediction/test_cases.rs](/model/src/systems/trajectory_prediction/test_cases.rs) | Rust | 112 | 0 | 23 | 135 |
| [model/src/systems/trajectory_update.rs](/model/src/systems/trajectory_update.rs) | Rust | 8 | 0 | 1 | 9 |
| [model/src/systems/warp_update_system.rs](/model/src/systems/warp_update_system.rs) | Rust | 53 | 4 | 11 | 68 |
| [model/src/util.rs](/model/src/util.rs) | Rust | 70 | 13 | 19 | 102 |
| [plan.md](/plan.md) | Markdown | 217 | 0 | 44 | 261 |
| [profile.json](/profile.json) | JSON | 1 | 0 | 0 | 1 |
| [saves/test.json](/saves/test.json) | JSON | 1 | 0 | 0 | 1 |
| [todo.md](/todo.md) | Markdown | 66 | 0 | 4 | 70 |
| [view/Cargo.toml](/view/Cargo.toml) | TOML | 17 | 0 | 3 | 20 |
| [view/resources/shaders/geometry.frag](/view/resources/shaders/geometry.frag) | OpenGL Shading Language | 5 | 0 | 2 | 7 |
| [view/resources/shaders/geometry.vert](/view/resources/shaders/geometry.vert) | OpenGL Shading Language | 15 | 0 | 3 | 18 |
| [view/resources/shaders/icon.frag](/view/resources/shaders/icon.frag) | OpenGL Shading Language | 9 | 0 | 3 | 12 |
| [view/resources/shaders/icon.vert](/view/resources/shaders/icon.vert) | OpenGL Shading Language | 18 | 0 | 3 | 21 |
| [view/src/events.rs](/view/src/events.rs) | Rust | 14 | 0 | 1 | 15 |
| [view/src/game.rs](/view/src/game.rs) | Rust | 52 | 0 | 11 | 63 |
| [view/src/game/camera.rs](/view/src/game/camera.rs) | Rust | 74 | 1 | 13 | 88 |
| [view/src/game/debug.rs](/view/src/game/debug.rs) | Rust | 28 | 0 | 8 | 36 |
| [view/src/game/debug/entities.rs](/view/src/game/debug/entities.rs) | Rust | 79 | 0 | 10 | 89 |
| [view/src/game/debug/overview.rs](/view/src/game/debug/overview.rs) | Rust | 6 | 0 | 1 | 7 |
| [view/src/game/input.rs](/view/src/game/input.rs) | Rust | 11 | 0 | 4 | 15 |
| [view/src/game/input/keyboard.rs](/view/src/game/input/keyboard.rs) | Rust | 24 | 0 | 8 | 32 |
| [view/src/game/input/mouse.rs](/view/src/game/input/mouse.rs) | Rust | 27 | 0 | 7 | 34 |
| [view/src/game/overlay.rs](/view/src/game/overlay.rs) | Rust | 45 | 0 | 5 | 50 |
| [view/src/game/rendering.rs](/view/src/game/rendering.rs) | Rust | 29 | 2 | 8 | 39 |
| [view/src/game/rendering/geometry_renderer.rs](/view/src/game/rendering/geometry_renderer.rs) | Rust | 34 | 0 | 8 | 42 |
| [view/src/game/rendering/shader_program.rs](/view/src/game/rendering/shader_program.rs) | Rust | 65 | 0 | 13 | 78 |
| [view/src/game/rendering/texture.rs](/view/src/game/rendering/texture.rs) | Rust | 26 | 0 | 4 | 30 |
| [view/src/game/rendering/texture_renderer.rs](/view/src/game/rendering/texture_renderer.rs) | Rust | 37 | 0 | 8 | 45 |
| [view/src/game/rendering/vertex_array_object.rs](/view/src/game/rendering/vertex_array_object.rs) | Rust | 71 | 0 | 12 | 83 |
| [view/src/game/resources.rs](/view/src/game/resources.rs) | Rust | 74 | 1 | 13 | 88 |
| [view/src/game/underlay.rs](/view/src/game/underlay.rs) | Rust | 15 | 0 | 3 | 18 |
| [view/src/game/underlay/celestial_objects.rs](/view/src/game/underlay/celestial_objects.rs) | Rust | 28 | 0 | 4 | 32 |
| [view/src/game/underlay/icons.rs](/view/src/game/underlay/icons.rs) | Rust | 124 | 14 | 19 | 157 |
| [view/src/game/underlay/icons/adjust_burn.rs](/view/src/game/underlay/icons/adjust_burn.rs) | Rust | 84 | 1 | 15 | 100 |
| [view/src/game/underlay/icons/burn.rs](/view/src/game/underlay/icons/burn.rs) | Rust | 76 | 0 | 12 | 88 |
| [view/src/game/underlay/icons/orbitable.rs](/view/src/game/underlay/icons/orbitable.rs) | Rust | 57 | 0 | 12 | 69 |
| [view/src/game/underlay/icons/vessel.rs](/view/src/game/underlay/icons/vessel.rs) | Rust | 57 | 0 | 13 | 70 |
| [view/src/game/underlay/segments.rs](/view/src/game/underlay/segments.rs) | Rust | 55 | 2 | 10 | 67 |
| [view/src/game/underlay/segments/orbit.rs](/view/src/game/underlay/segments/orbit.rs) | Rust | 51 | 3 | 10 | 64 |
| [view/src/game/underlay/segments/util.rs](/view/src/game/underlay/segments/util.rs) | Rust | 31 | 2 | 10 | 43 |
| [view/src/game/underlay/selected.rs](/view/src/game/underlay/selected.rs) | Rust | 144 | 8 | 24 | 176 |
| [view/src/game/util.rs](/view/src/game/util.rs) | Rust | 103 | 3 | 11 | 117 |
| [view/src/lib.rs](/view/src/lib.rs) | Rust | 7 | 0 | 1 | 8 |
| [view/src/menu.rs](/view/src/menu.rs) | Rust | 24 | 0 | 4 | 28 |
| [view/src/scenes.rs](/view/src/scenes.rs) | Rust | 2 | 0 | 0 | 2 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)