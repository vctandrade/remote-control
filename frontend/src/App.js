import React, { Component } from "react";

import Backward from './icons/backward.svg';
import Forward from './icons/forward.svg';
import Next from './icons/next.svg';
import Previous from './icons/previous.svg';
import Resume from './icons/resume.svg';
import VolumeDown from './icons/volume-down.svg';
import VolumeUp from './icons/volume-up.svg';

class App extends Component {
  handle_press(key) {
    return () => {
      var xhr = new XMLHttpRequest()

      xhr.open('POST', `http://${window.location.host}/api/press/${key}`)
      xhr.send()
    }
  }

  render() {
    return (
      <div className="vertical">
        <div className="horizontal">
          <img className="item" src={Previous} onClick={this.handle_press('page_down')} />
          <img className="item" src={Next} onClick={this.handle_press('page_up')} />
        </div>
        <div className="horizontal">
          <img className="item" src={Resume} onClick={this.handle_press('space')} />
        </div>
        <div className="horizontal">
          <img className="item" src={Backward} onClick={this.handle_press('left')} />
          <img className="item" src={Forward} onClick={this.handle_press('right')} />
        </div>
        <div className="horizontal">
          <img className="item" src={VolumeDown} onClick={this.handle_press('down')} />
          <img className="item" src={VolumeUp} onClick={this.handle_press('up')} />
        </div>
      </div>
    );
  }
}

export default App;
