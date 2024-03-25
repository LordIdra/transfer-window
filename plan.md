# Scenarios
## Tutorial
- ???

## Sandbox
- [1| ] Loading/saving games
- [1| ] Allow creating new celestial objects
- [1| ] Allow editing celestial objects
- [1| ] Allow spawning in enemy ships
- [1| ] Allow editing enemy ships
- eventually limited to tech you've unlocked in the campaign

## Campaign
- settlers in a new system, allowing some starting technology
- start on low-gravity planet so the low tech makes more sense
- subluminal interstellar travel
- alien riddles/puzzles
- natural disasters + space weather
- planets have unique resources, causes chokepoints etc

## Multiplayer
- PvP



# Non-sandbox scenarios
## Resource management
- building stuff takes time
- planetary industry
- interplanetary logistics
- space mining
- abstracted resources (metal, alloy, oil, etc) but not too fine detailed to prevent logistics hell
- resource synthesis (later game)
- processing chains for common components so they can be made on most planets
- atmospheric harvesting (including of stars)
- vessels can be recycled for resources
- planet resource deposits
- mining/drilling
- build ships/do manufacturing in space EVENTUALLY
- initial high cost of space manufacturing

## Tech tree
- research takes time
- research unlocked at points in the campaign
- unlocked research can be used in sandbox
- propulsion tech tree
- each ship has its own upgrade section (including 'large' upgrades)
- discovery system unlocks some tech nodes?
- players start with their own system?



# Information
## Fog of war
- line of site
- sensor strength
- radar/heat signatures
- transponders (not knowing what faction a ship belongs to could be cool)
- firing weapons allows shooter to be tracked more easily

## Communication
- tightbeams
- radio
- scanning from orbit
- communication relays

## Diplomacy
- other civilizations
- factions
- diplomacy
- dock to enemy ships/stations and board them to capture



# Celestial Objects
## Planets
- planet transport links (rail, sea, air)
- build around the outside of a planet
- space elevators
- WHAT TEXTURE TO PUT INSIDE PLANETS?
- Drag in atmosphere (oh god)
- atmosphere shaders (oh god)
- surface to orbit defensive installations (how does this change from planet to planet?)

## Asteroids
- asteroids can have orbits changed
- asteroid belt
- planet with non-orbitable rings?
- track only some asteroids at a time to prevent performance issues? (like KSP)



# LVs
- reusable LVs
- LV upgrades
- LV spaceplanes/shuttles
- LVs per planet, eg supermassive earth LVs would be extremely advanced/expensive
- manual LV control
- past very early game LVs can launch automatically



# Vessels
## General
- [1| ] System slots
- [1| ] System slots have types
- [1| ] Different vessels have different system slots
- ships/stations can be upgraded to another class
- spinning ships/stations to generate gravity (could make docking more interesting)
- docking

## Ships
### Types
- civilian
- transports
- tankers
- salvagers
- [1| ] Fighters
- [1| ] Frigates
- [2| ] destroyers
- cruisers
- battleships
- carriers
- dreadnoughts

### Ideas
- atmospheric flight
- bigger ships must be worth it because otherwise it turns into micromanaging small ships
- fleets
- ships have cargo space (warships obviously have less)

## Stations
### Types
- [1| ] Satellites (no function for now)
- [2| ] orbital defensive installations
- habitats
- shipyards
- off-world solar power

### Ideas
- off-planet industry
- modular stations
- station hull upgrades to fit new expansions
- O'neil cylinders

## Crew
- crew system
- crew consume resources
- crew transfer between ships/stations

## Systems
### Weaponry
- [1| ] Torpedoes
- [1| ] Point defense
- [1| ] Ballistic weaponry
- [2| ] directed energy weapons
- [2| ] railguns
- EMPs
- dazzlers
- nuclear weapons
- orbital strikes

### Propulsion
- [1| ] Chemical propulsion
- [1| ] Chemical fuel/lox reserves
- fission
- fusion
- solar sails
- accelerate objects with ground/space lasers

### Misc
- [2| ] shields
- ammunition
- automatic targeting
- salvaging

### Energy
- [2| ] solar panels
- [2| ] batteries
- [2| ] rtgs
- nuclear fission
- nuclear fusion

### Ideas
- [1| ] Torpedo guidance system (may be difficult)
- [1| ] PDC automatic targeting system
- [1| ] PDC range or time dropoff after which they are deleted to prevent bullet lag
- [1| ] PDC shells not affected by gravity
- [2| ] systems require energy to run
- [2| ] manage systems by turning them on/off eg radar
- solars/batteries degrade with time
- refurbish solars/batteries
- you can mix different energy generators
- more energy use when system in use, eg when railgun fires
- stronger propulsion causes irradiation
- manually guiding missiles at first
- shooting down missiles with missiles
- proximity detonation



# Debris
- debris fields
- debris indicators (shaders?)
- weapons that minimise debris
- kessler syndrome?
- debris cleanup



# UI
## Overlay
- [1| ] current time
- [1| ] time speed
- [1| ] Allow viewing N orbits into future on the selected orbit
- [2| ] UI of selected's burns
- [2| ] UI of all ships/stations you have (sorted by planet?)
- alert when new signature appears

## Underlay
- [1| ] Select a point on orbit
- [1| ] Warp to point on orbit
- [1| ] Click icon to focus object
- [1| ] Different types of objects have different icons
- [1| ] Overlapping icons render with a priority system
- [1| ] Right clicking vessels brings up context menu
- [1| ] Target option in the vessel menu selects vessel as 'target' relative to current vessel
- [1| ] Show closest encounter for target and selected
- [1| ] Deselect target when selected deselected (word salad)
- icons for fleets

## Ship management
- [1| ] Show current systems
- [1| ] Allow changing systems
- [1| ] Show current fuel reserves
- [1| ] Show remaining DV
- [1| ] Show orbital parameters
- [1| ] Lock weapons onto current target
- [1| ] Show current weapon lock
- hold fire setting

## Burn management
- [1| ] Burn can be adjusted (at all)
- [1| ] Smooth burn adjustment at different levels
- [1| ] Adjusting a burn with subsequent burns deletes them, but required confirmation by clicking unlock button
- [1| ] Hovering over unlock button shows text informing player that adjusting this burn will delete future burns (and click to confirm)
- [1| ] Visual indicator of how much DV burn will use
- [1| ] Burns using more DV than possible display a warning indicator
- [1| ] Warp to burn button

## Planet UI
- planet/moon distinction in UI titles

## Ideas
- [1| ] Better system for rendering conics
- [1| ] Refocus selected object
- [1| ] Drag to move focus
- [2| ] explosions (shaders?)
- icons to indicate ship/station/planet status
- some automation for being able to match orbits etc
