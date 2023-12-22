
# Note we are using an interface here as a workaroud for glue generation bug 
# if we try to define these in the platform they will get an unused error 
#  
# thread 'main' panicked at 'not yet implemented: Gracefully report 
# compilation problems during glue generation: [UnusedDef(`17.IdentId(2)`, @150-205)], []'

interface Game
    exposes [ColoredThings, RGBA, EngineCallins, Vec2, PluginState]
    imports []

ColoredThings : [Background, Wall, Paddle, Ball, Brick]
RGBA : {r:U8, g:U8, b:U8, a:U8}
# Point of PluginState is to store any sort of possible state information the plugin may need in its operation
# that is not explicitly provided by the engine.
# Plugin state should, in the best case, be opaque here (a box pointer or something like that) such that the
# plugin itself can define what it wants there (and can allocate appropriately). Not sure how to formulate that.
PluginState: {placeholder:U64}

Vec2: {x:F32, y:F32}

CallinReset : U64 -> PluginState
CallinColors: ColoredThings -> RGBA
CallinBounce: Vec2, Vec2 -> Vec2

# Callins are functions that engine can call when some specific event occurs in the engine,
# or when engine needs information from the plugin
EngineCallins : {
    # Called by engine to (re)set plugin's state
    # TODO: This should probably return a box (see comment about PluginState)
    # TODO: This technically does not need any arguments at all, but roc does not have void functions
    reset : CallinReset,
    # Called by engine to figure out colors for things
    colors : CallinColors,
    # Called by engine to get the bounce angle based on speed of paddle and speed of ball
    bounce: CallinBounce,
}

