#include <cassert>
#include <iostream>
#include <cmath>

// sum the first n elements of the array
int linearSum(int* A, int n) {
	if (n < 1) return 0;
	if (n == 1) {
		return A[0];
	} else {
		return linearSum(A, n - 1) + A[n - 1];
	}
}

// Ha! Who knew?
// ⌊x⌋ floor(x)
// ⌈y⌉ ceiling(y)

// Sum using binary recursion
// the depth of the recursion 
// (the maximum number of function instances that are active at the same time)
//  is 1 + log2
// thus uses an amount of additional space roughly proportional to this value
// running time is roughly proportional to n
// 2n - 1 boxes (in the diagram of the tree)

// i - starting index, n - length of sequence to be summed
int binarySum(int* A, int i, int n) {
	if (n == 1) {
		return A[i];
	} else {
		return binarySum(A, i, ceil(n/2)) + binarySum(A, i + ceil(n/2), floor(n/2));
	}
}


// reverse the elements of an array
// i - starting index, j ending at index inclusive
void reverseArray(int* A, int i, int j) {
	if (i < j) {
		int temp = A[i];
		A[i] = A[j];
		A[j] = temp;
		reverseArray(A, i + 1, j - 1);
	}
}

// non recursive of reverseArray
// i - starting index, j - ending index
void iterativeReverseArray(int* A, int i, int j) {
	while (i < j) {
		int temp = A[i];
		A[i] = A[j];
		A[j] = temp;
		i++;
		j--;
	}
}

// print the array so I can see it
void printArray(int A[], int length) {
	std::cout << "[";
	for (int i = 0; i < length; i++) {
		std::cout << A[i];
		if (i != length - 1) {
			std::cout << ", ";
		}
	}
	std::cout << "]";
}

int binaryFibonacci(int k) {
	if (k <= 1) {
		return k;
	} else {
		return binaryFibonacci(k - 1) + binaryFibonacci(k - 2);
	}
}

/*
int linearFibonacci(int k) {
	if (k <= 1) {
		return (k,0);
	} else {
		(i, j) <- linearFibonacci(k - 1);
		return (i +j, i);
	}
}*/

void floor() {
	float n = 2.4;
	std::cout << "floor(" << n << ") == " << floor(n) << std::endl;

	n = -2.4;
	std::cout << "floor(" << n << ") == " << floor(n) << std::endl;
	
	n = 2.5;
	std::cout << "floor(" << n << ") == " << floor(n) << std::endl;

	n = 2.4;
	std::cout << "ceil(" << n << ") == " << ceil(n) << std::endl;

	n = -2.4;
	std::cout << "ceil(" << n << ") == " << ceil(n) << std::endl;



}

int main() {

	floor();
	std::cout << std::endl;

	int a[4] = {1, 2, 3, 4};
	std::cout << "array: ";
	printArray(a, 4);
	std::cout << "\nsize: " << sizeof(a) / sizeof(int) << std::endl;
	std::cout <<  "sum: " << linearSum(a, 4) << std::endl;
	std::cout << "rev: ";
	reverseArray(a, 0, 3);
	printArray(a, 4);
	std::cout << std::endl;
	std::cout << std::endl;

	int b[5] = {4, 3, 6, 2, 5};
	std::cout << "array: ";
	printArray(b, 5);
	std::cout << "\nsize: " << sizeof(b) / sizeof(int) << std::endl;
	std::cout << "sum: " << linearSum(b, 5) << std::endl;
	reverseArray(b, 0, 4);
	std::cout << "rev: ";
	printArray(b, 5);
	std::cout << std::endl;
	std::cout << std::endl;


	int c[5] = {4, 3, 6, 2, 5};
	int d[5] = {4, 3, 6, 2, 5};
	std::cout << "array: ";
	printArray(c, 5);
	
	reverseArray(c, 0, 4);
	std::cout << "\nrev: ";
	printArray(c, 5);

	iterativeReverseArray(d, 0, 4);
	std::cout << "\nrev: ";
	printArray(d, 5);

	std::cout << "\nbinaryFibonacci(8) == " << binaryFibonacci(8) << std::endl;

	return 0;
}
