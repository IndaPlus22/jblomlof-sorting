# jblomlof-sorting

**NOTE:** You may need to update your rust version. The version I had (dont know which one) didnt work with the latest ggez release (0.8.1) there was some other crate (named zstd) that didn't work with the version I had. Now works without problems for rust version (1.65.0) 

Simply run main.rs with cargo run (make sure you have the rust 1.65.0 installed for 100% working, or not if you like to live dangerously) and it will show each algorithm in order and also loop. So only way to terminate is to close the window.  
The code for sorting algorithms is in sorters_vec.rs 
Originally I implemented the sorters for arrays but then I thought I needed vector sorters only, but I didnt really, I think. 
  
It's quite easy to change size and amount of elements to sort, just change GRID_CELL_SIZE and GRID_WIDTH. Amount is correlated to GRID_WIDTH. 

# Sorting algorithms
All sorting algorithms changes the orginal Vec\<isize\>, and returns a nested Vec\<Vec\<isize\>\> that contains all the steps the alg. did to sort it. So each vector in the return is a state the orignal vector was in.
 ### insert_sort
 Typical insert sort.
 ### selection_sort
 Typical selection sort.
 ### merge_sort
 Typical merge sort.
 ### yeet_sort
So it's really bad, but fun. It continously loops from start to end. If element[i] > element[i + 1], throw element[i] to the back. E.g if an element, a,  is greater than the next element put the element a at the back. It's very slow and could possible be faster if you throw in some reversing, you'll see.  
In reality it's just a bad selection_sort. 
