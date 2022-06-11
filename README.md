# Semester-Project
The Semester Project was done in the *Distributed Computing Laboratory* at the *School of Computer and Communication Sciences* at École polytechnique fédérale de Lausanne (EPFL).

The project deals with the authentication process required in [Carbon](https://github.com/Distributed-EPFL/carbon) and other cryptocurrencies.
It consists in the development of Merkle Patricia Tree with two main goals: 
* Give both *Proof-of-Inclusion* and *Proof-of-Exclusion* for every transaction in the batch (in the current implementation only the former is guaranteed). 
* Allow the brokers to send proofs to the clients, such that the minimum amount of information is efficiently sent over the network.

Here is the final report for further information: 
