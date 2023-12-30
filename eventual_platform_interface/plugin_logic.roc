app "roc_plugin"
    packages { pf: "platform/main.roc" }
    imports [pf.Game.{ColoredThings, RGBA,  Vec2,CallinColors,CallinBounce}]
    provides [callins] {State} to pf


State : { stateData : Str }

# Called by engine to (re)set plugin's state
reset : State
reset = {stateData:"Hello Stateful World"}


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

callins = {reset : reset, colors : selectColor, bounce : bounceAngle}
