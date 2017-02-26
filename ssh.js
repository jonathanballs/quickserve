var fs = require('fs');
var crypto = require('crypto');
var inspect = require('util').inspect;

var ssh2 = require('ssh2');
var utils = ssh2.utils;

function makeid()
{
    var text = "";
    var possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    for( var i=0; i < 7; i++ )
        text += possible.charAt(Math.floor(Math.random() * possible.length));

    return text;
}

function main()
{
    new ssh2.Server({
	hostKeys: [fs.readFileSync('host.key')]
    }, function(client) {
	console.log('Client connected!');

	client.on('authentication', function(ctx) {
	    ctx.accept();
	}).on('ready', function() {

	    client.on('session', function(accept, reject) {
		var session = accept();
		session.on('shell', function(accept, reject, info) {
		    console.log('Client is entering the shell');
		    var stream = accept();
                    var id = makeid();
                    stream.write("QuickServe: Your website can be found at http://quickserve.io/" + id);
                    client.id = id;
		});
	    });

	    client.on('request', function(accept, reject, name, info) {
		if (name === 'tcpip-forward') {
		    accept();
		    // Simulate an incoming connection
		    setTimeout(function() {
			console.log('Sending incoming tcpip forward');
			client.forwardOut(info.bindAddr,
				info.bindPort,
				'127.0.0.1', // Would normally come from a socket
				45678, // Would normally come from a socket
				function(err, stream) {
                                    //client.stream = stream;
				    if (err)
					return;
                                    stream.on('data', function(data) {console.log(data.toString()); stream.end()});
                                    stream.write("GET / HTTP/1.1\n\n");
				});
		    }, 1000);
		} else {
		    reject();
		}
	    })
	}).on('end', function() {
	    console.log('Client disconnected');
	});
    }).listen(2222, '127.0.0.1', function() {
	console.log('Listening on port ' + this.address().port);
    });
};
main()

module.exports = main

