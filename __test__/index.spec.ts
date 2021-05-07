import test from 'ava'

import { encode, decode, url_encode, url_decode } from '../index'

const b =
  'eyJ0aXRsZSI6Im9wL3MgaGlnaGVyIGlzIGJldHRlciIsImxlZ2VuZCI6W3sibmFtZSI6ImRvZ2UiLCJjb2xvciI6MTE2MjExMTl9LHsibmFtZSI6IndpbGx5IiwiY29sb3IiOi0xMDczMDc0MTc3fSx7Im5hbWUiOiJ2ZXRsaXgiLCJjb2xvciI6LTEyNjg0MDgzMjF9LHsibmFtZSI6ImRpY2VkIiwiY29sb3IiOi03OTE4MDgxfV0sInBvaW50cyI6W3sibmFtZSI6ImNvY2sgKGluKSIsInNjb3JlcyI6WzEwMDAsMjAwMCwwLjEsMTAwMF19XX0='
const buff = Buffer.from(b)
const o =
  '{"title":"op/s higher is better","legend":[{"name":"doge","color":11621119},{"name":"willy","color":-1073074177},{"name":"vetlix","color":-1268408321},{"name":"diced","color":-7918081}],"points":[{"name":"cock (in)","scores":[1000,2000,0.1,1000]}]}'
const str = Buffer.from(o)

test('encode function from native code', (t) => {
  t.is(encode(str), b)
})

test('decode function from native code', (t) => {
  t.is(decode(buff), o)
})

test('url safe encode function from native code', (t) => {
  t.is(url_encode(str), b)
})

test('url safe decode function from native code', (t) => {
  t.is(url_decode(buff), o)
})
