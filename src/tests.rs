use crate::*;

#[test]
fn empty_doc() {
    let doc = Document::new();
    assert_eq!(doc.elements().next(), None);
}

#[test]
fn character_dialogue_parenthesis_underline() {
    let text = "EDWARD
(shaking his head)
Jesus Christ.

WILL
Friend of yours?  Did you help him out of a bind?

EDWARD
Come on, Will.  Everyone likes that story.

WILL
No Dad, they don't.  _I_ do not like the story.  Not anymore, not after a _thousand_ _times_.  I know all the punchlines, Dad.  I can tell them as well as you can.
(closer)
For one night, one night in your entire life, the universe does not revolve around Edward Bloom.  It revolves around me and my wife.  How can you not understand that?";

    let doc = Document::from(text);
    let mut elements = doc.elements();
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "EDWARD");
        assert_eq!(e.elm_type(), ElmType::Character);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "(shaking his head)");
        assert_eq!(e.elm_type(), ElmType::Parenthetical);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "Jesus Christ.");
        assert_eq!(e.elm_type(), ElmType::Dialogue);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "WILL");
        assert_eq!(e.elm_type(), ElmType::Character);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(
            e.text(),
            "Friend of yours?  Did you help him out of a bind?"
        );
        assert_eq!(e.elm_type(), ElmType::Dialogue);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "EDWARD");
        assert_eq!(e.elm_type(), ElmType::Character);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "Come on, Will.  Everyone likes that story.");
        assert_eq!(e.elm_type(), ElmType::Dialogue);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "WILL");
        assert_eq!(e.elm_type(), ElmType::Character);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "No Dad, they don't.  _I_ do not like the story.  Not anymore, not after a _thousand_ _times_.  I know all the punchlines, Dad.  I can tell them as well as you can.");
        assert_eq!(e.elm_type(), ElmType::Dialogue);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "_I_");
        assert_eq!(e.elm_type(), ElmType::Underline);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "_thousand_");
        assert_eq!(e.elm_type(), ElmType::Underline);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "_times_");
        assert_eq!(e.elm_type(), ElmType::Underline);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "(closer)");
        assert_eq!(e.elm_type(), ElmType::Parenthetical);
    } else {
        unreachable!();
    }
    if let Some(e) = elements.next() {
        assert_eq!(e.text(), "For one night, one night in your entire life, the universe does not revolve around Edward Bloom.  It revolves around me and my wife.  How can you not understand that?");
        assert_eq!(e.elm_type(), ElmType::Dialogue);
    } else {
        unreachable!();
    }
    assert_eq!(elements.next(), None);
}
