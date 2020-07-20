import { factorial, reverse } from './pkg/rust_deno.js';

console.log( factorial( BigInt( 20 ) ) );

console.log( reverse( 'this is fun' ) );
