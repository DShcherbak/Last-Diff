#include <cmath>

int add(int x, int y) { return x + y;}

auto mult(int x, int y) -> int { return x * y;}

namespace HERE {
    void stable() {std::cout << "nothing." << std::endl;}

}


// Here is a comment

class Rem {
public:
    void removed() {}
};

void Rem::removed() {
    std::cout << "This function is removed." << std::endl;
}

enum class Enum {
    A,
    B,
    C
};


int hypothesis() {
    int a = 3;
    int b = 4;
    return std::sqrt(a * a + b * b);
}

int perform(int x) {
    int c = hypothesis();
    return x + c;
}