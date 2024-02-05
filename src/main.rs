use leptos::{svg::text, *};

pub struct Frame {
    pub left: isize,
    pub right: isize,
    /// The array before the recursive calls
    pub result: Option<Vec<i64>>,
    /// The final pivot index
    pub pivot: Option<isize>,
    /// The two child frames
    pub children: Option<Box<[Frame; 2]>>,
}

impl Frame {
    pub fn new_empty(left: isize, right: isize) -> Self {
        Self {
            left,
            right,
            result: None,
            pivot: None,
            children: None,
        }
    }

    pub fn new(
        left: isize,
        right: isize,
        result: Vec<i64>,
        pivot: isize,
        children: Box<[Frame; 2]>,
    ) -> Self {
        Self {
            left,
            right,
            result: Some(result),
            pivot: Some(pivot),
            children: Some(children),
        }
    }

    /// Counts the amount of frames
    pub fn count(&self) -> usize {
        1 + self
            .children
            .as_ref()
            .map(|children| children[0].count() + children[1].count())
            .unwrap_or(0)
    }

    /// Gets the highest recursion depth
    pub fn max_depth(&self, depth: usize) -> usize {
        let depth = depth + 1;
        match &self.children {
            Some(children) => children[0]
                .max_depth(depth)
                .max(children[1].max_depth(depth)),
            _ => depth,
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.children.push(
            view! { <text x=10 y=renderer.y class="text">"qS("{self.left}", "{self.right}", ...)"</text> }
                .into_view(),
        );
        renderer.y += 100;
        if let Some(children) = &self.children {
            children[0].render(renderer);
            children[1].render(renderer);
        }
    }
}

pub struct Renderer {
    pub children: Vec<View>,
    pub y: u64,
    pub max_depth: usize,
    pub frame_count: usize,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            y: 50,
            max_depth: 0,
            frame_count: 0,
        }
    }

    pub fn render(&mut self, frame: &Frame) {
        self.max_depth = self.max_depth.max(frame.max_depth(0));
        self.frame_count += frame.count();
        frame.render(self);
    }

    pub fn view_box(&self) -> String {
        format!("0 0 {} {}", self.max_depth * 300, self.frame_count * 100)
    }

    pub fn finish(self) -> impl IntoView {
        view! {
            <svg viewBox=self.view_box()>
                <style>
                r"
                .text {
                    font: 40px mono;
                    fill: blue;
                    dominant-baseline: middle;
                }
                "
                </style>
                {self.children}
            </svg>
        }
    }
}

pub fn quick_sort(array: &mut [i64], left: isize, right: isize) -> Frame {
    if right <= left {
        return Frame::new_empty(left, right);
    }
    let pivot = array[right as usize];
    let mut i = left;
    let mut j = right - 1;
    while i < j {
        while array[i as usize] < pivot && i < right {
            i += 1;
        }
        while array[j as usize] > pivot && j > left {
            j -= 1;
        }
        if i < j {
            array.swap(i as usize, j as usize);
        }
    }
    array.swap(i as usize, right as usize);
    let result = array.to_vec();
    let children = Box::new([
        quick_sort(array, left, i - 1),
        quick_sort(array, i + 1, right),
    ]);
    Frame::new(left, right, result, i, children)
}

#[component]
fn App() -> impl IntoView {
    let frame = quick_sort(&mut [3, 5, 2, 7, 8, 6, 1, 9, 3, 4], 0, 9);
    let mut renderer = Renderer::new();
    renderer.render(&frame);
    renderer.finish()
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
