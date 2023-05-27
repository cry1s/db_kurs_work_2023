import React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Login from './pages/Login';
import AdminPage from './pages/AdminPage';
import PatientPage from './pages/PatientPage';

const App: React.FC = () => {
  return (
    <Router>
      <Routes>
        <Route  path="/" Component={Login} />
        <Route path="/admin" Component={AdminPage} />
        <Route path="/patient" Component={PatientPage} />
      </Routes>
    </Router>
  );
};

export default App;