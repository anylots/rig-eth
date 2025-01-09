import React from 'react';  

const Header: React.FC = () => {  
  return (  
    <header className="fixed top-0 left-0 w-full bg-gray-800 bg-opacity-75 text-white py-4 px-6 shadow-md z-50">  
      <div className="max-w-7xl mx-auto flex justify-between items-center">  
        <h1 className="text-xl font-bold">AI Agent App</h1>  
        <nav>  
          <a href="/" className="mr-4 hover:underline">  
            Home  
          </a>  
          <a href="/create" className="hover:underline">  
            Create  
          </a>  
        </nav>  
      </div>  
    </header>  
  );  
};  

export default Header;