# **Project Overview**

This project provides implementations of various sorting algorithms in the Rust language. Enabled sorting algorithms:

- Bubble Sort
- Insertion Sort
- Merge Sort
- Quick Sort
- Selection Sort



### **Usage**
Below are examples of using the code.


First, let's start with a simple example and write an array of natural numbers.
![image](https://github.com/iandi2002/Rust_/assets/79205166/1756ec68-543e-48bc-a3a5-3c6294c1e465)

You can see that all the sorting methods work correctly, and all have the same result.

![image](https://github.com/iandi2002/Rust_/assets/79205166/7d0f1544-46fe-463e-8c48-de0d38b785fb)



Now the screenshot shows an example of a different number order.
![image](https://github.com/iandi2002/Rust_/assets/79205166/6018eb82-0e85-4e49-888a-6d22dfb58559)

In this example, the answers are also displayed correctly.

![image](https://github.com/iandi2002/Rust_/assets/79205166/2b4b35ca-9556-4b2e-aa20-54916595e88a)


An example of using negative numbers.

![image](https://github.com/iandi2002/Rust_/assets/79205166/92bad379-8265-494e-a39e-655fc172e5e5)

All methods sorted the numbers without errors.

![image](https://github.com/iandi2002/Rust_/assets/79205166/a7fc21e8-2d54-4441-840a-1cafecce6859)




This example shows how sorting methods sort an array that contains duplicate numbers and a zero.
![image](https://github.com/iandi2002/Rust_/assets/79205166/f15a4cd9-2418-4eae-b94d-2d1f5cae9777)

The same correct answer for all methods.

![image](https://github.com/iandi2002/Rust_/assets/79205166/c3c93027-dd73-499a-b660-11168289ee71)






### **Examples**
Some code snippets will be shown below and their importance will be described in detail.

#### **Bubble Sort**

Bubble sorting is a simple sorting algorithm that iterates through a list, comparing pairs of adjacent items and swapping them if they are in the wrong order. This process continues until the list is sorted.

![image](https://github.com/iandi2002/Rust_/assets/79205166/f3ec338b-0486-428a-985b-20f112e363b1)


#### **Insertion Sort**

Insertion sorting is a sorting algorithm that goes through a list of elements, alternately moving each element to its correct position. During each iteration, the algorithm takes another element and inserts it into the sorted part of the list.

![image](https://github.com/iandi2002/Rust_/assets/79205166/f8680f64-2267-4e10-a2ca-c7e81b450d66)


#### **Merge Sort**

Merge sorting is an efficient sorting algorithm that uses the principle of "divide and conquer". It splits the list into two equal parts, recursively sorts each part, and then merges them together into a sorted list.

![image](https://github.com/iandi2002/Rust_/assets/79205166/a2579d5e-1132-4910-b1d7-698b624cd6aa)


#### **Quick Sort**

Quick sort is an efficient sorting algorithm that also uses the principle of "divide and conquer". It selects a reference element from the list, splits the list into two parts around that element, recursively sorts each part and merges them together.

![image](https://github.com/iandi2002/Rust_/assets/79205166/da284b45-a7df-4846-b164-aa886c020e76)


#### **Selection Sort**

Selection sorting is a simple sorting algorithm that finds the minimum element in a list and exchanges it with the first element, then finds the next minimum element and exchanges it with the second element, and so on until the entire list is sorted.

![image](https://github.com/iandi2002/Rust_/assets/79205166/8a02d148-9f6e-46d1-b844-3418054c7d36)


#### **Connecting**
These methods are prescribed in lib.rs

![image](https://github.com/iandi2002/Rust_/assets/79205166/5ef6d00a-bc93-4d93-b436-64110b84a117)

Adding the possibility of using these methods in main.rs

![image](https://github.com/iandi2002/Rust_/assets/79205166/f2476395-1cac-4fbd-85ab-bc09374059e5)







