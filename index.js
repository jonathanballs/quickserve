const express = require('express'),
      app = express()

//settings
app.set('view engine', 'pug')
app.use(express.static('static'))

//opening port 8000 for http
app.listen(8000, function()
{
    var greeting = "________        .__        __                                      \r\n\\_____  \\  __ __|__| ____ |  | __  ______ ______________  __ ____  \r\n \/  \/ \\  \\|  |  \\  |\/ ___\\|  |\/ \/ \/  ___\/\/ __ \\_  __ \\  \\\/ \/\/ __ \\ \r\n\/   \\_\/.  \\  |  \/  \\  \\___|    <  \\___ \\\\  ___\/|  | \\\/\\   \/\\  ___\/ \r\n\\_____\\ \\_\/____\/|__|\\___  >__|_ \\\/____  >\\___  >__|    \\_\/  \\___  >\r\n       \\__>             \\\/     \\\/     \\\/     \\\/                 \\\/ ";
    console.log(greeting)
    console.log("Listening on port 8000")
})


//routes
app.get("/s/*", function(req,res)
{
    //handle the creation of HTTP requests
    var method = req.method;
    var path = req.url.substring(3, req.url.length)

    res.send(method + " " + path)
})

app.get("/", function(req,res)
{
    res.render('index')
})
