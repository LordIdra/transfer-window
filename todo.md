# Current tasks
[x] MVC split
[x] Rendering backend
[x] Primitive planet rendering as sphere
[x] Refocus selected object
[x] Drag to move focus
[x] Load/save games
[x] Display current time
[x] Speed up / slow down time
[x] Build default model
[x] Render icons (different types for different objects)
[x] Click icon to focus object
[x] Overlapping icons render with a priority system
[x] Intelligent conic rendering
[x] Update orbit state
[x] Select a point on orbit
[x] Display warp speed
[x] Show time of selected point
[x] Warp to point on orbit
[x] Can create burn
[x] Burn icons
[x] Select burns by default
[x] Prioritise currently selected icon
[x] Burn can be adjusted (at all)
[x] Better burn adjustment system
[x] Intelligent burn rendering
[x] Fast solver for burns
[x] Fix burn adjustment logic not triggering when over icon??
[x] Fix adjusting burn while spacecraft encounters it causing crash
[x] Fix only granular adjustments to burn
[x] Warp to burn button
[x] Add FPS counter
[x] Fix being able to create burns for orbitables
[x] Make segment point selection snap to nearest segment
[x] View N orbits into future on the selected orbit
[x] Fix freezes when focus enters burn
[x] Show scale somewhere (how long 1m, 1k, etc is)
[x] Fix clicking another burn point causes state change on selected one
[x] Visual indicator of how much DV burn will use
[x] Fix icons showing for burns that are underway
[x] The next segment should take precedence over orbits when rendering (ie render segments backwards)
[x] Allow deleting burns
[x] Fix selection breaking down on high eccentricity orbits
[x] More logical time step system
[x] Adjusting/deleting/creating a burn with subsequent burns cannot be done, other burns must be deleted first
[x] Better color scheme for orbits/burns
[x] Limit prediction by number of conics as well as time
[x] Periapsis/apoapsis indicators
[x] Fix current segment does not exist nonsense
[x] Just fade icons when overlapped instead of hiding completely 
[x] Completely deselect segment point when UI hovered
[x] Add logging to find/fix issues with solver, it keeps panicking
[x] Right clicking vessels brings up context menu
[x] Target option in the vessel menu selects vessel as 'target' relative to current vessel
[x] Target is highlighted/circled
[x] Show closest encounter for target and selected
[x] Allow vessels to be deleted without crashing everything (oh god)
[x] Add ship classes with system slot schema
[x] Add system slots for propulsion + fuel tanks
[x] Add light/heavy ships
[x] Add fuel tanks
[x] Add engines
[x] Create GUI for editing system slots and add ship layout texture
[x] Show current systems in system slots
[x] Allow editing current systems
[x] System tooltips
[x] Close button for vessel editor
[x] Show remaining DV + fuel bar in vessel top left GUI
[x] Disallow changing modules to change burns
[x] Vessel mass should account for remaining fuel
[x] Cannot create new burns if no engines or out of fuel
[x] Reduce DV/fuel while burning
[x] Add torpedo system
[x] Show active weapons at bottom of screen (simple for now)
[x] Click button to confirm fire
[x] Allow deselecting objects
[x] Show target on all selected objects
[x] Can adjust initial trajectory before firing
[x] Preview of initial trajectory
[x] Spawn new torpedo at ship location with trajectory
[x] Allow deleting and warping to fire torpedo events
[x] Update deletion logic for burns and launch events
[x] Disallow creating events before last event
[x] Torpedo stockpile
[x] Torpedoes have some velocity by default so they're not just buried under the ship icon
[x] Min/max zoom levels....
[x] Icons per vessel class
[x] Icons face vessel velocity
[x] Torpedo PN guidance system -> consider entire trajectory for now ie no fog or war
[x] Visualise guidance segment
[x] Actually determine whether guidance can be enabled at selected point
[x] Guidance icon
[x] Cancel guidance
[x] Fix closest point weirdness??
[x] Fix backward burn dragging
[x] Fix or remove burn scrolling?
[x] Preload images to fix weird loading delay in editor
[x] Better collision detection
[x] Collision event which yeets both vessels
[x] Draw orbit after guidance end only if miss
[x] Recompute guidance segment if collision no longer happening
[x] Allow cancelling timeline events before collision event
[x] Fix being able to MODIFY (not yeet) torpedo fire events when torpedo has events queued
[x] Select new burn/guidance/etc created
[x] Intercept icon
[x] Allow cancelling burns/guidance midway through
[x] Bloom
[x] Burns cannot use more DV than ship has including previous burns
[x] Disallow creating burns/guidances after dv zero
[x] Show DV/fuel used by burn (+ numbers for start/end)
[x] Show burn duration
[x] Close right click menu on option selected
[x] Tooltips to label buttons
[x] Images instead of MDI icons in UI
[x] Replace most buttons with icons and add tooltips
[x] Disallow warp if within 5 seconds of the thing
[x] Overhaul vessel editor UI
[x] Fix E X P A N S I O N of editor UI
[x] Fix texture blending to bring main texture to the front
[x] Encounter icons
[x] Don't count some icons as hovered/clicked
[x] Ignore child elements when zoomed out far enough
[x] Planet/moon/star enums + icons
[x] Explosion shaders
[x] Celestial object UI
[x] Fix being unable to drag arrows???
[x] Finish encounter visual timeline events
[x] Allow selecting apoapsis, intercept, approach, encounter
[x] Switch to GL_LINES for segment rendering (should be massive speedup)
[x] Show velocity/altitude of vessel + points + celestial objects
[x] Add actual logging
[x] Log to external file
[x] Select event when clicked in timeline
[x] Visual timeline for selected events + points
[x] Current burn/guidance finishes (exclude intercepts)
[x] Show orbital parameters
[x] Factions (player, ally, enemy)
[x] Can't see future enemy movements
[x] Can't edit enemy vessels/timeline
[x] fix 5 km lol
[x] fix weird closest distance formatting (repro around moon?)
[x] Can't select orbitable ap/pe
[x] Select vessel/orbitable for points/apoapsis/etc
[x] Fix being able to intercept planets
[x] Tooltips for speed on warp arrows
[x] Investigate/handle crashes at model/src/api/trajectories/fast_solver/bounding/ellipse.rs:26:102
[x] Handle ITP failure crashes
[x] Persistent selected state (doesn't deselect on time changed)
[x] Allow selecting burn + guidance points?
[x] Fix backward adjustment
[x] Explorer tree
[x] Fix very low/high eccentricity orbits...
[x] Fix the weird thing where you can modify torpedo trajectory after intercept
[x] Scout
[x] Frigate
[x] Tiny fuel tank
[x] Small fuel tank
[x] Medium fuel tank (frigate+)
[x] Regular engine
[x] Efficient engine (frigate+)
[x] Booster engine (frigate+)
[x] Torpedo launcher
[x] Enhanced torpedo launcher
[x] Close button for vessel editor
[x] Add station vessel class (can't be edited, no engines)
[x] Station icon
[x] Docking ports
[x] Docking
[x] Undocking
[x] Draw resources
[x] Fix section divider in UI
[x] Resource transfer
[x] Resource transfer can be cancelled
[x] The vessel system is pretty insane, separate in TorpedoVessel etc traits or even option structs! (effectively mini ECS)
[x] Vessel system still uses weird mix of single/multiple slot system
[x] Fuel facility
[x] Engine facility
[x] Torpedo launcher facility
[x] Torpedo storage facility
[x] Docking facility
[x] Class refactor
[x] Redraw ships w/ more and smaller slots
[x] Weapons UI redesign (it's shit)
[x] Weapon cooldown
[x] Equipment adds mass
[x] More logical ordering of docking ports
[x] More logical ordering in explorer
[x] Menu screen
[x] Storyteller triggers
[x] Storyteller events
[x] Storyteller dialogue
[x] 1-01
[x] Vessel constructor
[x] Orbitable constructor
[x] Storyteller objectives
[x] Persist completed levels
[x] Different texture for completed/locked levels
[x] Hide aspects of UI
[x] Exit to menu option
[x] 1-02



# ------------------------------------ #
# CAMPAIGN PLANNING AREA
# ------------------------------------ #
# Backend
- Remember to upgrade ship and unlock new equipment
- Satellite which loses power every so often, small window to intercept, reverse orbit direction

# Levels
[ ] 03 Hohmann
  - start in low circular orbit, want to get to higher circular orbit
  - only need to use tangent arrows (normal next time)
  - create elliptical orbit to start with target apoapsis
  - create another burn at apoapsis and circularise
  - delta-v

[ ] 04 Chase
  - set target
  - explanation of closest approaches
  - catch up to a spacecraft in front
  - then drop back to a spacecraft behind
  - get within X km

- 05 Translunar
  - spheres of influence
  - first intercept not necessarily the best, keep going
  - hyperbolic orbits (aaaaaaaaaaaaaaaaaa) (will it come back down? it won't! there's a cutoff point)
  - fly by the moon

MAYBE NOT FROM HERE TO END - JUST DO IN CAMPAIGN MISSIONS, BUT WE'LL SEE HOW IT GOES
- 06 Moon Orbit
  - open ended
  - orbit the moon and come back into LEO

- 07 Rendezvous
  - use your knowledge of closest approaches to get an intercept
  - then try and match its orbit
  - patience! this might be more difficult than the last few missions
  - dock

- 08 Station
  - takes place entirely at allied hub
  - upgrade to frigate
  - allied ship also docked
  - equip new frigate
  - resource transfers
  - torpedo transfers
  - we'll be using those torpedoes next time

- 09 Destroy
  - fire torpedoes
  - guidance explanation
  - destroy dummy targets
  - target behind and in front - is there an advantage to being in front / behind?

- 10 Evade
  - evade enemies torpedo
  - can't evade that one, time to deploy our own torpedo against it, but leaves us without offensive capability...
  - the end, time for real life

## Optional Levels
[ ] Scientific notation
[ ] Orbital parameters
[ ] ISP
[ ] Kepler's equation
[ ] N-Body simulations (vs patched conics)
[ ] Rocket fuels overview
[ ] Proportional guidance
[ ] Integration techniques (euler RK4 etc)
[ ] Joules, watts, energy



# ------------------------------------ #
# TECHNICAL AREA
# ------------------------------------ #
# Performance
[ ] Texture atlas
[ ] Switch to RK4 for burn/guidance integration (allows lower time step as well)
[ ] Computing closest encounters on terminal hyperbola orbits is extremely slow, maybe model as straight lines beyond certain range depending on mission design, or other restrictions?
[ ] Switch to Pade approximation of EKE for better performance in singular corner (https://www.sciencedirect.com/science/article/pii/S0094576522005999)
[ ] Smaller screen texture renderer framebuffer

# Technical debt tracker
- Docking port view logic is horrible, needs more cleanup when we have more transfers maybe
- Timeline view logic is not great
- Hardcoded end time lol



# ------------------------------------ #
# PROJECTS AREA
# ------------------------------------ #
# Projects
[ ] Planet textures/generation, atmosphere shaders?
[ ] Proper planet texture sourcing
[ ] Collisions with planets (nearly forgot about that lmao)
[ ] Ballistic weaponry
[ ] Point defense
[ ] PDC automatic targeting system
[ ] PDC range or time dropoff after which they are deleted to prevent bullet lag
[ ] PDC shells not affected by gravity?
[ ] Different fuel/lox mixtures
[ ] Energy production/storage/consumptiongit pull
[ ] Main menu background GIFs

# Before release
[ ] Versioning!
[ ] Icon/branding
[ ] Dev mode compiles in debug menu
[ ] Profiling + performance testing on different devices
[ ] Test compatibility on different devices
[ ] Website
[ ] Add licenses
[ ] Log to multiple files



# ------------------------------------ #
# SPECULATION AREA
# ------------------------------------ #
# Artificial Intelligence
- 3 independent neural networks
- targeting
- navigation
- guidance

# Comms
- Comms equipment
- Comms line overlay
- Comms routing algorithm based on line of sight

# Radar
- Fog of war
- Passive/active radar
- Information networking