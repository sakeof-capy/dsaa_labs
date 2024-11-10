#include <algorithm>
#include <iostream>
#include <cmath>
#include <functional>
#include <vector>
#include <thread>

using Number = int;
using Iterator = typename std::vector<Number>::iterator;
using Order = bool;

static constexpr Order ASCENDING = true;
static constexpr Order DESCENDING = !ASCENDING;

enum class Error
{
    Success,
    NotPowerOfTwoSize,
};

static constexpr auto cmp = [](Number a, Number b) -> bool {
    return a < b;
};

bool is_power_of_two(std::size_t number) 
{
    return (number & (number - 1)) == 0;
}

void merge(const Iterator begin, const Iterator end, Order order)
{
    const auto count = (end - begin);
    const auto mid_ndx = count / 2;

    if (count <= 1)
    {
        return;
    }    

    const Iterator mid = begin + mid_ndx;

    for (Iterator it = begin; it != mid; ++it)
    {
        if (!cmp(*it, *(it + mid_ndx)) == order)
        {
            std::swap(*it, *(it + mid_ndx));
        }
    }
    
    merge(begin, mid, order);
    merge(mid, end, order);
}

void batchers_sort_utility(Iterator begin, Iterator end, Order order, std::size_t parallelizations_left)
{
    auto count = (end - begin);

    if (count <= 1)
    {
        return;
    }

    Iterator mid = begin + count / 2;
    if (parallelizations_left > 0)
    {
        std::thread thread1([begin, mid, parallelizations_left, order] {
            batchers_sort_utility(begin, mid, order, parallelizations_left - 1);
        });
        std::thread thread2([begin, mid, end, parallelizations_left, order] {
            batchers_sort_utility(mid, end, !order, parallelizations_left);
        });

        thread1.join();
        thread2.join();
    }
    else
    {
        batchers_sort_utility(begin, mid, order, parallelizations_left);
        batchers_sort_utility(mid, end, !order, parallelizations_left);
    }

    merge(begin, end, order);
}

Error batchers_sort(std::vector<Number>& vec, std::size_t threads_count)
{
    if (!is_power_of_two(vec.size()))
    {
        return Error::NotPowerOfTwoSize;
    }

    std::size_t parallelizations_count = static_cast<std::size_t>(std::log2(threads_count));
    batchers_sort_utility(vec.begin(), vec.end(), ASCENDING, parallelizations_count);

    return Error::Success;
}

void print(const std::vector<Number>& vec)
{
    std::cout << "[ ";

    for (Number n : vec)
    {
        std::cout << n << ' '; 
    }

    std::cout << "]" << '\n';
}

std::vector<int> generate_random_vector(int N, int min = 0, int max = 100) {
    std::vector<int> vec(N);
    srand(static_cast<unsigned>(time(0)));

    for (int i = 0; i < N; ++i) {
        vec[i] = min + rand() % (max - min + 1);
    }
    return vec;
}

template<typename ReturnType>
ReturnType measure_time(std::function<ReturnType()>&& func) {
    auto start = std::chrono::high_resolution_clock::now();
    ReturnType result = func();
    auto end = std::chrono::high_resolution_clock::now();   // End timing

    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    std::cout << "Execution time: " << duration.count() << " milliseconds" << std::endl;

    return result;
}

int main()
{
    const std::size_t MAX_PRINTABLE_SIZE = 1024;
    const std::size_t THREADS_COUNT = 4;
    const std::size_t ARRAY_SIZE = 1024 * 1024 * 64;
    std::vector<Number> array = generate_random_vector(ARRAY_SIZE );
    std::vector<Number> parallel_array = array;

    if (array.size() <= MAX_PRINTABLE_SIZE)
    {
        std::cout << "Initial array to be sequentially sorted: ";
        print(array);

        std::cout << "Initial array to be sorted in parallel: ";
        print(parallel_array);
    }

    std::cout << "Sequential sort:\n";
    const Error err1 = measure_time<Error>([&array] {
        return batchers_sort(array, 1);
    });

    std::cout << "Parallel sort:\n";
    const Error err2 = measure_time<Error>([&parallel_array] {
        return batchers_sort(parallel_array, THREADS_COUNT);
    });

    if (err1 == Error::NotPowerOfTwoSize || err2 == Error::NotPowerOfTwoSize) {
        std::cout << "Invalid size: " << array.size() << '\n';
        return EXIT_FAILURE;
    }

    if (array.size() <= MAX_PRINTABLE_SIZE)
    {
        std::cout << "Sequentially sorted array: \n";
        print(array);

        std::cout << "Sorted in parallel array: \n";
        print(parallel_array);
    }

    if (std::ranges::is_sorted(array) && std::ranges::is_sorted(parallel_array))
    {
        std::cout << "Both arrays are sorted.\n";
    }

    std::cout.flush();
    return EXIT_SUCCESS;
}
