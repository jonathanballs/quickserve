const express = require('express'),
      app = express(),
      ssh = require('./ssh')

//settings
app.set('view engine', 'pug')
app.use(express.static('static'))

//opening port 8000 for http
app.listen(8000, function()
{
    console.log("Listening on port 8000")
})


//routes
app.get("/", function(req,res)
{
    res.render('index')
})
