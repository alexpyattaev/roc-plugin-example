platform "breakout-plugin"
# How do I require more than one function here?
    requires {} { main : ColoredThings -> RGBA }
    exposes []
    packages {}
    imports [Game.{ColoredThings, RGBA}]
    # Why can I not provide more than one thing here?
    provides [mainForHost]

mainForHost : Str -> RGBA
mainForHost = \argFromHost ->
    when argFromHost is
        "BACKGROUND_COLOR" -> main Background
        "WALL_COLOR" -> main Wall
        "PADDLE_COLOR" -> main Paddle
        "BALL_COLOR" -> main Ball
        "BRICK_COLOR" -> main Brick
        _ -> crash "the host provided an unexpected color key value"

testForHost: I64 -> I64
testForHost = \x -> 
    x + 1
