# Producer and consumer problem

The producer and consumer problem demonstrates synchronization problem between producers and consumers. Producers generate data and place it in the shared buffer. Consumers remove and process the data from the buffer.
This problem demonstrates how different processes can share data without conflicts.

Challenge is to ensure that:

* A producer does not add data to a full buffer
* A consumer does not remove data from an empty buffer
* Multiple producers and consumers don't access the buffer simultaneously, that prevents race conditions

More about the problem can be found here: https://www.geeksforgeeks.org/operating-systems/producer-consumer-problem-using-semaphores-set-1/

