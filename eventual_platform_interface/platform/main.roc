platform "breakout-plugin"
    requires {State} { callins : _ }
    exposes []
    packages {}
    imports [Game.{ColoredThings, RGBA, Vec2, CallinColors, CallinBounce}]

    provides [mainForHost]

# Point of State is to store any sort of possible state information the plugin may need in its operation
# that is not explicitly provided by the engine.
# Plugin state should, in the best case, be opaque here (a box pointer or something like that) such that the
# plugin itself can define what it wants there (and can allocate appropriately). Not sure how to formulate that.


# Callins are functions that engine can call when some specific event occurs in the engine,
# or when engine needs information from the plugin
EngineCallins : {
    # Called by engine to (re)set plugin's state
    reset : State as Reset,
    # Called by engine to figure out colors for things
    colors : CallinColors,
    # Called by engine to get the bounce angle based on speed of paddle and speed of ball
    bounce: CallinBounce,
}
mainForHost : EngineCallins
mainForHost = callins



