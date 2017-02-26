const express = require('express'),
      app = express(),
      request = require('request'),
      ssh = require('./ssh.js')

//settings
app.set('view engine', 'pug')
app.use(express.static('static'))


//opening port 8000 for http
app.listen(8000, function()
    {
    var greeting = " ________        .__        __                                      \r\n\\_____  \\  __ __|__| ____ |  | __  ______ ______________  __ ____  \r\n \/  \/ \\  \\|  |  \\  |\/ ___\\|  |\/ \/ \/  ___\/\/ __ \\_  __ \\  \\\/ \/\/ __ \\ \r\n\/   \\_\/.  \\  |  \/  \\  \\___|    <  \\___ \\\\  ___\/|  | \\\/\\   \/\\  ___\/ \r\n\\_____\\ \\_\/____\/|__|\\___  >__|_ \\\/____  >\\___  >__|    \\_\/  \\___  >\r\n       \\__>             \\\/     \\\/     \\\/     \\\/                 \\\/ "
    console.log(greeting)
    console.log("Listening on port 8000")
    })


//routes
app.get("/s/*", function(req,res)
    {
    //handle the creation of HTTP requests
    var method = req.method;
    var path = req.url.substring(3, req.url.length)
    var slug = path.substring(0, path.indexOf("/"))
    var path = "/"+path.substring(path.indexOf("/")+1, path.length)
    //MAKING THE REQUEST
    // request('http://localhost:'+slug+path, function(response,body) {
    //     console.log(body)
    //     res.send(body.body)
    // })
    var found = false;
    var tmp;
    ssh.clientList.forEach(function(client)
        {
            if(client.id == slug)
            {
                found = true;
                tmp = client
            }
        })
    if(found)
    {
        tmp.stream.write(makeHTTP(method,path)); 
        console.log(makeHTTP(method,path))
        var buffer = "";
        tmp.stream.on('data',function(data)
            {
                buffer += data.toString() 
            })
        tmp.stream.on('close',function()
            {
                res.send(buffer)
            })
    }
    else
    {
        res.send("Error 1, client not found")
    }
})

app.get("/", function(req,res)
    {
    res.render('index')
    })

function makeHTTP(method, path)
{
    return method + " "+path + " HTTP/1.1\n\n"
}
