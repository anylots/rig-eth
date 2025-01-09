import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';  
import MainPage from './MainPage';  
import CreatePage from './CreatePage';  
import Header from './Header';  

function App() {  
  return (  
    <Router>  
      <Header />  
      <div className="pt-20 flex justify-center items-center min-h-screen bg-gray-100">  
        <Routes>  
          <Route path="/" element={<MainPage />} />  
          <Route path="/create" element={<CreatePage />} />  
        </Routes>  
      </div>  
    </Router>  
  );  
}  

export default App;