initSidebarItems({"fn":[["from_raw_parts","Forms a slice from a pointer and a length."],["from_raw_parts_mut","Performs the same functionality as `from_raw_parts`, except that a mutable slice is returned."],["from_ref","Converts a reference to T into a slice of length 1 (without copying)."],["from_ref_mut","Converts a reference to T into a slice of length 1 (without copying)."]],"mod":[["memchr","Pure rust memchr implementation, taken from rust-memchr"]],"struct":[["Chunks","An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a time)."],["ChunksMut","An iterator over a slice in (non-overlapping) mutable chunks (`chunk_size` elements at a time). When the slice len is not evenly divided by the chunk size, the last slice of the iteration will be the remainder."],["ExactChunks","An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a time)."],["ExactChunksMut","An iterator over a slice in (non-overlapping) mutable chunks (`chunk_size` elements at a time). When the slice len is not evenly divided by the chunk size, the last up to `chunk_size-1` elements will be omitted."],["Iter","Immutable slice iterator"],["IterMut","Mutable slice iterator."],["RSplit","An iterator over subslices separated by elements that match a predicate function, starting from the end of the slice."],["RSplitMut","An iterator over the subslices of the vector which are separated by elements that match `pred`, starting from the end of the slice."],["RSplitN","An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits, starting from the end of the slice."],["RSplitNMut","An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits, starting from the end of the slice."],["Split","An iterator over subslices separated by elements that match a predicate function."],["SplitMut","An iterator over the subslices of the vector which are separated by elements that match `pred`."],["SplitN","An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits."],["SplitNMut","An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits."],["Windows","An iterator over overlapping subslices of length `size`."]],"trait":[["SliceExt","Extension methods for slices."],["SliceIndex","A helper trait used for indexing operations."]]});