import "./styles/header.css";

const Header = ({ children }) => {
  return (
    <div className="messenger-header">
      <h1>{children}</h1>
    </div>
  );
};

export { Header };