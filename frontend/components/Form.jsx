import React from 'react';
import PropTypes from 'prop-types';

export default function Form({ onSubmit, currentUser }) {
  return (
    <form onSubmit={onSubmit}>
      <fieldset id="fieldset">
        <p>Add or update your status message!</p>
        <p className="highlight">
          <label htmlFor="message">Message:</label>
          <input
            autoComplete="off"
            autoFocus
            id="message"
            required
          />
        </p>
        <button type="submit">
          Update
        </button>
      </fieldset>
    </form>
  );
}

Form.propTypes = {
  onSubmit: PropTypes.func.isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired
  })
};
