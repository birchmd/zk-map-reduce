# ZK Map Reduce

A silly demo to show how zero knowledge proofs enable delegating computation to untrusted actors.

This is a web application to compute the sum of the first 10 square numbers. The clients perform the squaring computation (the job ID indicates what number they are doing), i.e. the "map" part of the map-reduce, and submits to the server. The calculation comes with a ZK proof which the server verifies. Clients can choose to be dishonest which causes them to send faulty proofs that the server will reject. Once the server has correct proofs for all 10 results, it will perform the summation of those results (i.e. the "reduce" part of the map-reduce) to get the final answer.
