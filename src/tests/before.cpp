int main() {
    int x = 0; int y = 3;
    int z = x     +y;
    return 0;
}

int mult(int x, int y) { return x * y; }

int add(int x, int y) {return x + y;       }

void stable() {std::cout << "nothing." << std::endl;}

void removed() {}
 
int perform(int x) {
    int a = 3;
    int b = 4;
    int c = std::sqrt(a * a + b * b);

    return x + c;
}