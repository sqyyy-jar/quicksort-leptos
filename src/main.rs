use leptos::*;

pub struct Frame {
    pub left: isize,
    pub right: isize,
    pub full: Option<FullFrame>,
}

pub struct FullFrame {
    /// The array before the recursive calls
    pub result: Vec<i64>,
    /// The final pivot index
    pub pivot: isize,
    /// The two child frames
    pub children: Box<[Frame; 2]>,
}

impl Frame {
    pub fn new_empty(left: isize, right: isize) -> Self {
        Self {
            left,
            right,
            full: None,
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
            full: Some(FullFrame {
                result,
                pivot,
                children,
            }),
        }
    }

    /// Counts the amount of frames
    pub fn count(&self) -> usize {
        1 + self
            .full
            .as_ref()
            .map(|full| full.children.iter().map(Frame::count).sum())
            .unwrap_or(0)
    }

    /// Gets the highest recursion depth
    pub fn max_depth(&self, depth: usize) -> usize {
        let depth = depth + 1;
        match &self.full {
            Some(FullFrame { children, .. }) => children[0]
                .max_depth(depth)
                .max(children[1].max_depth(depth)),
            _ => depth,
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        let full = if let Some(full) = &self.full {
            view! {
                {full.result.iter().enumerate().map(|(i, &column)| {
                    view! {
                        <text x=(400 + i * 50) y=renderer.y class="text anchor-middle">{column}</text>
                    }
                }).collect_view()}
                <circle cx=(400 + full.pivot * 50) cy=renderer.y class="circle" />
            }
            .into_view()
        } else {
            ().into_view()
        };
        renderer.children.push(
            view! {
                <text x=(10 + renderer.depth * 25) y=renderer.y class="text">"qS("{self.left}", "{self.right}", ...)"</text>
                {full}
            }
            .into_view(),
        );
        renderer.y += 50;
        renderer.depth += 1;
        if let Some(full) = &self.full {
            full.children[0].render(renderer);
            full.children[1].render(renderer);
        }
        renderer.depth -= 1;
    }
}

pub struct Renderer {
    pub children: Vec<View>,
    pub y: u64,
    pub max_depth: usize,
    pub frame_count: usize,
    pub depth: usize,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            y: 25,
            max_depth: 0,
            frame_count: 0,
            depth: 0,
        }
    }

    pub fn render(&mut self, frame: &Frame) {
        self.max_depth = self.max_depth.max(frame.max_depth(0));
        self.frame_count += frame.count();
        frame.render(self);
    }

    pub fn view_box(&self) -> String {
        format!(
            "0 0 {} {}",
            800 + self.max_depth * 25,
            self.frame_count * 50
        )
    }

    pub fn finish(self) -> impl IntoView {
        view! {
            <svg viewBox=self.view_box()>
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
    view! {
        <div class="app">
            {renderer.finish()}
        </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <style>{include_str!("style.css")}</style>
            <App/>
        }
    });
}
