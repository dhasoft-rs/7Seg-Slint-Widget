 # TFRT 02 - 7Seg-Slint-Widget

The goal of this project is to create a ‘7-segment display’ Custom Widget. All the elements that will make up the GUI are defined in the app-window.slint file.

<p align="center">
  <img width="250" src="/7SegWidget.png">
  &nbsp; &nbsp; &nbsp;
  <img width="250" src="/7SegEdition.png">
</p>

The application contains 2 buttons (-) and (+), the 3x7segment display and a TextInput for directly defining the value to be displayed.

<p>&nbsp;</p>

# Principle

The display will show the value of an internal counter, range between 0 and 999. It is made up of 3 images ('hundreds', 'tens' and 'units') whose source will change depending on the counter value. The counter value (initially 489) can either be modified using the 2 buttons, or edited directly by clicking on the display (that will show a TextInput widget), then confirmed by pressing _[ENTER]_.

<p>&nbsp;</p>

# Inside app-window.slint file

- Buttons are defined by the **MyButton** component.
- The display (3 images of a 7-segment display) and the TextInput widget are defined in the **MyDisplay** component.
- **ImgData** component contains pictures for the 10 digits. This is not a graphics component as such, but rather an ‘array’ of images.
- The main **AppWindow** component integrates the (-) button, the display and the (+) button in a horizontal layout.

<p>&nbsp;</p>

# Operating

The overall program flow can be found in the HorizontalLayout part of the **AppWindow** component.
- First comes the ‘decButton’, which decrements the counter on its _‘clicked’_ event.
- Next comes the ‘disp’ component, which contains the display and the TextInput for modifying counter value. It updates the images according to the counter value, manages the display of the TextInput widget on the _‘clicked’_ event, manages the filtering of characters entered on the _‘entry_edited’_ event, and manages the validation of the value entered on the _‘entry_accepted’_ event. 
- Finally, you'll find the ‘incButton’, which increments the counter on its _‘clicked’_ event.
<p>&nbsp;</p>

The main.rs file contains the functions called during the events described just above.
If you need more ‘basic’ informations on how to design a project with Slint, and about how interactions work between the Slint part (*.slint file) and the Rust code (main.rs file), please have a look at the first project of this series, which you can find here: https://github.com/dhasoft-rs/Introducing-Slint/blob/main/README.en.md .

That's all for this second Topic, enjoy :-)
