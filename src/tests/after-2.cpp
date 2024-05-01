int add(int x, int y) {
    return x + y;
}

auto mult(int x, int y) -> int {
    return x * y; 
}

void stable() {
    std::cout << "nothing." << std::endl;
}

int hypothesis()
{
  int a = 3;
  int b = 4;
  return std::sqrt(a * a + b * b);
}

int perform(int x) 
{
  int c = hypothesis();
  return x + c;
}