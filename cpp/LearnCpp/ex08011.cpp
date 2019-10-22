#include <iostream>

class HelloWorld
{
private:
	char *m_data;

public:
	HelloWorld()
	{
		m_data = new char[14];
		const char *init = "Hello, World!";
		for (int i = 0; i < 14; ++i)
			m_data[i] = init[i];
	}

	~HelloWorld()
	{
          delete [] m_data;
        // replace this comment with your destructor implementation
	}

	void print() const
	{
		std::cout << m_data;
	}

};

int main()
{
	HelloWorld hello;
	hello.print();

    return 0;
}
