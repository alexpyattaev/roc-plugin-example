app "roc_plugin"
    packages { pf: "platform/main.roc" }
    imports [pf.Game.{ColoredThings, RGBA, EngineCallins, Vec2, PluginState,CallinReset,CallinColors,CallinBounce}]
    provides [main] to pf





# Called by engine to (re)set plugin's state
reset : CallinReset
reset = \_arg->
    {placeholder:42}

reset2 : CallinReset
reset2 = \_arg->
    crash "This is broken on purpose"


# Called by engine to get the bounce angle based on speed of paddle and speed of ball
bounceAngle : CallinBounce
bounceAngle =\paddle, ball ->
    {x:paddle.x + ball.x, y:paddle.y +ball.y}


# Called by engine to figure out colors for things
selectColor:CallinColors
selectColor= \color ->
    when color is 
        Background -> {r:255, g:0, b:0, a:255}
        Wall -> {r:255, g:255, b:0, a:255}
        Paddle -> {r:255, g:0, b:255, a:255}
        Ball -> {r:0, g:0, b:255, a:255}
        Brick -> {r:0, g:255, b:0, a:255}

main: U64->EngineCallins
main = \arg ->
    when arg is 
        0 -> {reset : reset, colors : selectColor, bounce : bounceAngle}
        _ -> {reset : reset2, colors : selectColor, bounce : bounceAngle}
