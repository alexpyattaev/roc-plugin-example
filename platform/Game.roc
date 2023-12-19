
# Note we are using an interface here as a workaroud for glue generation bug 
# if we try to define these in the platform they will get an unused error 
#  
# thread 'main' panicked at 'not yet implemented: Gracefully report 
# compilation problems during glue generation: [UnusedDef(`17.IdentId(2)`, @150-205)], []'
interface Game
    exposes [ColoredThings, RGBA]
    imports []

ColoredThings : [Background, Wall, Paddle, Ball, Brick]
RGBA : {r:U8, g:U8, b:U8, a:U8}