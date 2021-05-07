const { encode, decode } = require('./index')

const str =
  'eyJ0aXRsZSI6Im9wL3MgaGlnaGVyIGlzIGJldHRlciIsImxlZ2VuZCI6W3sibmFtZSI6InBldGl0aW8iLCJjb2xvciI6MTE2MjExMTl9LHsibmFtZSI6Im5vZGUtZmV0Y2giLCJjb2xvciI6LTEwNzMwNzQxNzd9LHsibmFtZSI6ImdvdCIsImNvbG9yIjotMTI2ODQwODMyMX1dLCJwb2ludHMiOlt7Im5hbWUiOiIyIGJ5dGVzIiwic2NvcmVzIjpbMTYzNDksNjAxMSwyMTE5XX1dfQ==.png'
const buff = Buffer.from(
  str,
  //'base64',
)
console.log(encode(buff))
console.log(decode(Buffer.from(encode(buff))), Buffer.from(str, 'base64').toString('binary'))
