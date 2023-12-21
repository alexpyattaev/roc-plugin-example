app "roc_plugin"
    packages { pf: "platform/main.roc" }
    imports []
    provides [main, test] to pf

main = \color ->
    when color is 
        Background -> {r:255, g:0, b:0, a:255}
        Wall -> {r:255, g:255, b:0, a:255}
        Paddle -> {r:255, g:0, b:255, a:255}
        Ball -> {r:0, g:0, b:255, a:255}
        Brick -> {r:0, g:255, b:0, a:255}

# So I'd like this to be callable from the engine... I'm providing this to pf, but pf seems to be unable to import it!
test: I64 -> I64
test = \x ->
    x + 1
