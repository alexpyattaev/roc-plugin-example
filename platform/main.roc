platform "breakout-plugin"

    requires {} { main : U64 -> EngineCallins }
    exposes []
    packages {}
    imports [Game.{ColoredThings, RGBA, EngineCallins, Vec2, PluginState,CallinReset,CallinColors,CallinBounce}]

    provides [mainForHost]

mainForHost : U64 -> EngineCallins
mainForHost = \argFromHost ->
    main argFromHost


