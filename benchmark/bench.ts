import b from 'benny'

import { decode, encode } from '../index'

const buff = Buffer.from(
  'eyJ0aXRsZSI6Im9wL3MgaGlnaGVyIGlzIGJldHRlciIsImxlZ2VuZCI6W3sibmFtZSI6ImRvZ2UiLCJjb2xvciI6MTE2MjExMTl9LHsibmFtZSI6IndpbGx5IiwiY29sb3IiOi0xMDczMDc0MTc3fSx7Im5hbWUiOiJ2ZXRsaXgiLCJjb2xvciI6LTEyNjg0MDgzMjF9LHsibmFtZSI6ImRpY2VkIiwiY29sb3IiOi03OTE4MDgxfV0sInBvaW50cyI6W3sibmFtZSI6ImNvY2sgKGluKSIsInNjb3JlcyI6WzEwMDAsMjAwMCwwLjEsMTAwMF19XX0=',
)
const buff2 =
  'eyJ0aXRsZSI6Im9wL3MgaGlnaGVyIGlzIGJldHRlciIsImxlZ2VuZCI6W3sibmFtZSI6ImRvZ2UiLCJjb2xvciI6MTE2MjExMTl9LHsibmFtZSI6IndpbGx5IiwiY29sb3IiOi0xMDczMDc0MTc3fSx7Im5hbWUiOiJ2ZXRsaXgiLCJjb2xvciI6LTEyNjg0MDgzMjF9LHsibmFtZSI6ImRpY2VkIiwiY29sb3IiOi03OTE4MDgxfV0sInBvaW50cyI6W3sibmFtZSI6ImNvY2sgKGluKSIsInNjb3JlcyI6WzEwMDAsMjAwMCwwLjEsMTAwMF19XX0='
const str = Buffer.from(decode(buff))
const str2 = decode(buff)
async function run1() {
  await b.suite(
    'decode base64',

    b.add('napi', () => {
      decode(buff)
    }),

    b.add('atob', () => {
      atob(buff2)
    }),

    b.cycle(),
    b.complete(),
  )
}

async function run2() {
  await b.suite(
    'encode base64',

    b.add('napi', () => {
      encode(str)
    }),

    b.add('btoa', () => {
      btoa(str2)
    }),

    b.cycle(),
    b.complete(),
  )
}

run1().catch((e) => {
  console.error(e)
})

run2().catch((e) => {
  console.error(e)
})
