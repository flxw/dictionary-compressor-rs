First version:
- read whole file
- go through list of words
- put each word into a hashmap
- replace word with map key
- write out compressed file along with decompression map

Second version:
- stream file word by word

Third version:
- improve key range after determining number of dictionary entries
