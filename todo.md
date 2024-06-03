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

# Polish
[ ] Allow selecting apoapsis, periapsis, closest approach, encounter, and warping to them
[ ] Show upcoming segments including timeline events
[ ] Some way to see distance/altitude and time at apoapsis/periapsis/encounters/approaches
[ ] Show orbit/burn/guidance parameters

# Campaign
[ ] Factions (player, ally, enemy)
[ ] Can't see future enemy movements
[ ] Menu screen
[ ] Stations
[ ] Can only change equipment at stations
[ ] Satellite class
[ ] Trainer class
[ ] Scout class
- Remember to upgrade ship and unlock new equipment
- Satellite which loses power every so often, small window to intercept, reverse orbit direction

# Bigger picture
[ ] Planet textures/generation, atmosphere shaders?
[ ] Ballistic weaponry
[ ] Point defense
[ ] PDC automatic targeting system
[ ] PDC range or time dropoff after which they are deleted to prevent bullet lag
[ ] PDC shells not affected by gravity?
[ ] Collisions with planets (nearly forgot about that lmao)
[ ] Energy production/storage/consumption
[ ] Sandbox mode - EVENTUALLY as model versions may be a problem

# Backburner
[ ] Switch to Pade approximation of EKE for better performance in singular corner (https://www.sciencedirect.com/science/article/pii/S0094576522005999)
[ ] Computing closest encounters on terminal hyperbola orbits is extremely slow, maybe model as straight lines beyond certain range depending on mission design, or other restrictions?

# Before release
[ ] Add actual logging
[ ] Versioning!
[ ] Profiling + performance testing on different devices
[ ] Test compatibility on different devices
[ ] Website
[ ] Add licenses