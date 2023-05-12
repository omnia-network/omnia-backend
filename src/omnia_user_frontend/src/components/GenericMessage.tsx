import React from 'react';

import './GenericMessage.css';

interface IProps {
  message: string;
}

const GenericMessage: React.FC<IProps> = ({ message }) => {

  if (!message) {
    return null;
  }

  return (
    <div className="generic-message">
      {message}
    </div>
  );
};

export default GenericMessage;
