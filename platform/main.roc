platform "breakout-plugin"
    requires {} { main : ColoredThings -> RGBA }
    exposes []
    packages {}
    imports [Game.{ColoredThings, RGBA}]
    provides [mainForHost]

mainForHost : Str -> RGBA
mainForHost = \argFromHost ->
    when argFromHost is
        "BACKGROUND_COLOR" -> main Background
        "WALL_COLOR" -> main Wall
        "PADDLE_COLOR" -> main Paddle
        "BALL_COLOR" -> main Ball
        "BRICK_COLOR" -> main Brick
        _ -> crash "unreachable - host provided an unexpected argument"